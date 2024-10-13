use super::utils::centered_rect;
use super::widgets::popup::Popup;
use std::io;

use crate::db::connection::establish_connection;
use crate::models::Task;
use diesel::SqliteConnection;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, List, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};

enum CurrentScreen {
    MainScreen,
    Deleting,
}

pub struct App {
    conn: SqliteConnection,
    tasks: Vec<Task>,
    task_state: ListState,
    current_screen: CurrentScreen,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut conn = establish_connection();
        let tasks = Task::list(&mut conn).unwrap();
        Self {
            conn,
            tasks,
            task_state: ListState::default().with_selected(Some(0)),
            current_screen: CurrentScreen::MainScreen,
            exit: false,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.current_screen {
            CurrentScreen::MainScreen => self.handle_main_screen_key_event(key_event),
            CurrentScreen::Deleting => self.handle_deleting_screen_key_event(key_event),
        }
    }

    fn handle_main_screen_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => self.refresh_tasks(),
            KeyCode::Char('j') => self.task_state.select_next(),
            KeyCode::Char('k') => self.task_state.select_previous(),
            KeyCode::Char('d') => self.start_task_deletion(),
            _ => {}
        }
    }

    fn handle_deleting_screen_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('y') => {
                self.delete_selected_task();
                self.current_screen = CurrentScreen::MainScreen;
            }
            KeyCode::Char('n') => self.current_screen = CurrentScreen::MainScreen,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    fn refresh_tasks(&mut self) {
        self.tasks = Task::list(&mut self.conn).unwrap();
    }

    fn delete_selected_task(&mut self) {
        if let Some(selected) = self.task_state.selected() {
            let task = self.tasks.remove(selected);
            Task::delete(&mut self.conn, task.id).unwrap();
        }
    }

    fn start_task_deletion(&mut self) {
        self.current_screen = CurrentScreen::Deleting;
    }

    fn render_task_list(&mut self, area: Rect, buf: &mut Buffer) {
        let task_list = List::new(&self.tasks)
            .block(Block::bordered().title("Tasks"))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        StatefulWidget::render(task_list, area, buf, &mut self.task_state);
    }

    fn render_task_detail(&self, area: Rect, buf: &mut Buffer) {
        if let Some(selected) = self.task_state.selected() {
            let task = &self.tasks[selected];
            let text = vec![
                Line::from(format!("Title: {}", task.title)),
                Line::from(format!(
                    "Description: {}",
                    task.description.as_deref().unwrap_or("")
                )),
                Line::from(format!("Status: {}", task.status)),
            ];
            Paragraph::new(text)
                .block(Block::bordered().title("Details"))
                .render(area, buf);
        }
    }

    fn render_confirm_deletion(&self, area: Rect, buf: &mut Buffer) {
        let text = vec![
            Line::from("Are you sure you want to delete this task?"),
            Line::from("Press 'y' to confirm or 'n' to cancel"),
        ];
        Popup::default()
            .title(Line::from("Confirm Deletion"))
            .content(text)
            .render(area, buf);
    }
}
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, detail_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        self.render_task_list(main_area, buf);
        self.render_task_detail(detail_area, buf);
        if let CurrentScreen::Deleting = self.current_screen {
            let popup_area = centered_rect(40, 10, area);
            self.render_confirm_deletion(popup_area, buf);
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
