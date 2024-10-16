use super::components::popup::Popup;
use super::components::project_list::ProjectList;
use super::components::task_list::TaskList;
use super::components::Component;
use super::utils::centered_rect;
use crate::db::connection::establish_connection;
use ratatui::Frame;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal,
};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

enum ActiveScreen {
    Projects,
    Tasks,
}

pub struct App {
    tasks: TaskList,
    projects: ProjectList,
    active_screen: ActiveScreen,
    popup: Option<Popup>,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let conn = Rc::new(RefCell::new(establish_connection()));
        Self {
            tasks: TaskList::new(Rc::clone(&conn)),
            projects: ProjectList::new(Rc::clone(&conn)),
            active_screen: ActiveScreen::Tasks,
            popup: None,
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
        let [main_area, detail_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(frame.area());
        let [task_area, project_area] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area);
        frame.render_widget(&mut *self, detail_area);
        self.tasks.render(frame, task_area);
        self.projects.render(frame, project_area);
        if let Some(popup) = self.get_popup() {
            let popup_area = centered_rect(60, 20, frame.area());
            popup.render(frame, popup_area);
        }
    }

    fn get_popup(&mut self) -> Option<&mut Popup> {
        if self.popup.is_some() {
            return self.popup.as_mut();
        }

        if let Some(popup) = self.projects.popup.take() {
            self.popup = Some(popup);
        } else if let Some(popup) = self.tasks.popup.take() {
            self.popup = Some(popup);
        }

        self.popup.as_mut()
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
        if let Some(popup) = self.popup.take() {
            self.handle_popup_key_event(key_event, popup);
        } else {
            match self.active_screen {
                ActiveScreen::Tasks => self.handle_tasks_key_event(key_event),
                ActiveScreen::Projects => self.hannle_projects_key_event(key_event),
            }
        }
    }

    fn handle_tasks_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Tab => self.active_screen = ActiveScreen::Projects,
            _ => self.tasks.handle_key_events(key_event),
        }
    }

    fn hannle_projects_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Tab => self.active_screen = ActiveScreen::Tasks,
            _ => self.projects.handle_key_events(key_event),
        }
    }

    fn handle_popup_key_event(&mut self, key_event: KeyEvent, mut popup: Popup) {
        match key_event.code {
            KeyCode::Enter => {
                popup.submit();
                self.refresh();
                self.popup = None
            }
            KeyCode::Esc => self.popup = None,
            _ => {
                popup.handle_key_events(key_event);
                self.popup = Some(popup)
            }
        }
    }

    // fn handle_deleting_screen_key_event(&mut self, key_event: KeyEvent) {
    //     match key_event.code {
    //         KeyCode::Char('y') => {
    //             self.delete_selected_task();
    //             self.active_screen = CurrentScreen::MainScreen;
    //         }
    //         KeyCode::Char('n') => self.active_screen = CurrentScreen::MainScreen,
    //         _ => {}
    //     }
    // }

    fn exit(&mut self) {
        self.exit = true;
    }
    fn refresh(&mut self) {
        self.tasks.refresh();
        self.projects.refresh()
    }

    // fn delete_selected_task(&mut self) {
    //     if let Some(selected) = self.task_state.selected() {
    //         let task = self.tasks.remove(selected);
    //         Task::delete(&mut self.conn.borrow_mut(), task.id).unwrap();
    //     }
    // }
    //
    // fn start_task_deletion(&mut self) {
    //     self.active_screen = CurrentScreen::Deleting;
    // }

    fn render_task_detail(&self, area: Rect, buf: &mut Buffer) {
        if let Some(selected_task) = self.tasks.get_selected() {
            let text = vec![
                Line::from(format!("Title: {}", selected_task.title)),
                Line::from(format!(
                    "Description: {}",
                    selected_task.description.as_deref().unwrap_or("")
                )),
                Line::from(format!("Status: {}", selected_task.status)),
            ];
            Paragraph::new(text)
                .block(Block::bordered().title("Task details"))
                .render(area, buf);
        }
    }

    fn render_project_detail(&self, area: Rect, buf: &mut Buffer) {
        if let Some(selected_project) = self.projects.get_selected() {
            let text = vec![
                Line::from(format!("Title: {}", selected_project.title)),
                Line::from(format!(
                    "Description: {}",
                    selected_project.description.as_deref().unwrap_or("")
                )),
                Line::from(format!("Status: {}", selected_project.status)),
            ];
            Paragraph::new(text)
                .block(Block::bordered().title("Project details"))
                .render(area, buf);
        }
    }

    // fn render_confirm_deletion(&self, area: Rect, buf: &mut Buffer) {
    //     let text = vec![
    //         Line::from("Are you sure you want to delete this task?"),
    //         Line::from("Press 'y' to confirm or 'n' to cancel"),
    //     ];
    //     Popup::default()
    //         .title(Line::from("Confirm Deletion"))
    //         .content(text)
    //         .render(area, buf);
    // }
}
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.active_screen {
            ActiveScreen::Tasks => {
                self.render_task_detail(area, buf);
            }
            ActiveScreen::Projects => {
                self.render_project_detail(area, buf);
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
