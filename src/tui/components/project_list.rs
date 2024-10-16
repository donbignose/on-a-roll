use std::{cell::RefCell, rc::Rc};

use diesel::SqliteConnection;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};

use crate::models::Project;

use super::{
    list_selection::ListSelection, popup::Popup, project_input::ProjectInput,
    project_update::ProjectUpdate, Component,
};

pub struct ProjectList {
    conn: Rc<RefCell<SqliteConnection>>,
    projects: ListSelection<Project>,
    pub popup: Option<Popup>,
}

impl ProjectList {
    pub fn new(conn: Rc<RefCell<SqliteConnection>>) -> Self {
        let projects = Project::list(&mut conn.borrow_mut()).unwrap();
        Self {
            conn: Rc::clone(&conn),
            projects: ListSelection::new(projects, "Projects"),
            popup: None,
        }
    }
    pub fn get_selected(&self) -> Option<&Project> {
        self.projects.selected()
    }

    fn handle_list_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('a') => {
                self.popup = Some(Popup::ProjectInput(ProjectInput::new(Rc::clone(
                    &self.conn,
                ))));
            }
            KeyCode::Char('u') => {
                if let Some(selected_project) = self.projects.selected() {
                    self.popup = Some(Popup::ProjectUpdate(ProjectUpdate::from_project(
                        Rc::clone(&self.conn),
                        selected_project,
                    )));
                }
            }
            KeyCode::Char('d') => {
                // self.mode = Mode::Delete;
            }
            _ => self.projects.handle_key_events(key),
        }
    }
    // fn handle_add_key_events(&mut self, key: KeyEvent) {
    //     match key.code {
    //         KeyCode::Enter => {
    //             self.project_input.submit_and_reset();
    //             // self.mode = Mode::List;
    //             self.refresh();
    //         }
    //         KeyCode::Esc => {
    //             // self.mode = Mode::List;
    //         }
    //         _ => self.project_input.handle_key_events(key),
    //     }
    // }
    // fn handle_update_key_events(&mut self, key_event: KeyEvent) {
    //     if let Some(project_update) = &mut self.project_update {
    //         match key_event.code {
    //             KeyCode::Esc => {
    //                 // self.mode = Mode::List;
    //                 self.project_update = None;
    //             }
    //             KeyCode::Enter => {
    //                 project_update.submit_and_reset();
    //                 self.project_update = None;
    //                 // self.mode = Mode::List;
    //                 self.refresh();
    //             }
    //             _ => project_update.handle_key_events(key_event),
    //         }
    //     }
    // }
    pub fn refresh(&mut self) {
        self.projects
            .set_items(Project::list(&mut self.conn.borrow_mut()).unwrap());
    }
}

impl Component for ProjectList {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        self.projects.render(f, area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        self.handle_list_key_events(key)
    }
}
