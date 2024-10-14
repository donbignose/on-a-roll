use super::components::task_input::TaskInput;
use super::widgets::popup::Popup;
use super::{components::Component, utils::centered_rect};
use crate::db::connection::establish_connection;
use crate::models::Task;
use diesel::SqliteConnection;
use ratatui::Frame;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, List, ListState, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

enum CurrentScreen {
    MainScreen,
    Deleting,
    TaskInput,
}

pub struct App {
    conn: Rc<RefCell<SqliteConnection>>,
    tasks: Vec<Task>,
    task_state: ListState,
    current_screen: CurrentScreen,
    task_input: TaskInput,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let conn = Rc::new(RefCell::new(establish_connection()));
        let tasks = Task::list(&mut conn.borrow_mut()).unwrap();
        Self {
            conn: Rc::clone(&conn),
            tasks,
            task_state: ListState::default().with_selected(Some(0)),
            current_screen: CurrentScreen::MainScreen,
            task_input: TaskInput::new(Rc::clone(&conn)),
            exit: false,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(&mut *self, frame.area());
        if let CurrentScreen::TaskInput = self.current_screen {
            self.task_input
                .render(frame, centered_rect(50, 30, frame.area()));
        }
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
            CurrentScreen::TaskInput => self.handle_task_input_key_event(key_event),
        }
    }

    fn handle_main_screen_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => self.refresh_tasks(),
            KeyCode::Char('j') => self.task_state.select_next(),
            KeyCode::Char('k') => self.task_state.select_previous(),
            KeyCode::Char('d') => self.start_task_deletion(),
            KeyCode::Char('a') => self.start_task_input(),
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

    fn handle_task_input_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.current_screen = CurrentScreen::MainScreen;
            }
            KeyCode::Enter => {
                self.task_input.create_task_and_reset();
                self.current_screen = CurrentScreen::MainScreen;
                self.refresh_tasks();
            }
            _ => self.task_input.handle_key_events(key_event),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    fn refresh_tasks(&mut self) {
        self.tasks = Task::list(&mut self.conn.borrow_mut()).unwrap();
    }

    fn delete_selected_task(&mut self) {
        if let Some(selected) = self.task_state.selected() {
            let task = self.tasks.remove(selected);
            Task::delete(&mut self.conn.borrow_mut(), task.id).unwrap();
        }
    }

    fn start_task_deletion(&mut self) {
        self.current_screen = CurrentScreen::Deleting;
    }
    fn start_task_input(&mut self) {
        self.current_screen = CurrentScreen::TaskInput;
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
