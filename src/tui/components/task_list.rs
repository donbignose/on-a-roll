use std::{cell::RefCell, rc::Rc};

use diesel::SqliteConnection;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};

use crate::models::Task;

use super::{
    list_selection::ListSelection, popup::Popup, task_input::TaskInput, task_update::TaskUpdate,
    Component,
};

pub struct TaskList {
    conn: Rc<RefCell<SqliteConnection>>,
    tasks: ListSelection<Task>,
    pub popup: Option<Popup>,
}

impl TaskList {
    pub fn new(conn: Rc<RefCell<SqliteConnection>>) -> Self {
        let tasks = Task::list(&mut conn.borrow_mut()).unwrap();
        Self {
            conn: Rc::clone(&conn),
            tasks: ListSelection::new(tasks, "Tasks"),
            popup: None,
        }
    }
    pub fn get_selected(&self) -> Option<&Task> {
        self.tasks.selected()
    }

    fn handle_list_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('a') => {
                self.popup = Some(Popup::TaskInput(TaskInput::new(Rc::clone(&self.conn))));
            }
            KeyCode::Char('u') => {
                if let Some(selected_task) = self.tasks.selected() {
                    self.popup = Some(Popup::TaskUpdate(TaskUpdate::from_task(
                        Rc::clone(&self.conn),
                        selected_task,
                    )));
                }
            }
            KeyCode::Char('d') => {
                // self.mode = Mode::Delete;
            }
            _ => self.tasks.handle_key_events(key),
        }
    }
    // fn handle_add_key_events(&mut self, key: KeyEvent) {
    //     match key.code {
    //         KeyCode::Enter => {
    //             self.task_input.submit_and_reset();
    //             // self.mode = Mode::List;
    //             self.refresh();
    //         }
    //         KeyCode::Esc => {
    //             // self.mode = Mode::List;
    //         }
    //         _ => self.task_input.handle_key_events(key),
    //     }
    // }
    // fn handle_update_key_events(&mut self, key_event: KeyEvent) {
    //     if let Some(task_update) = &mut self.task_update {
    //         match key_event.code {
    //             KeyCode::Esc => {
    //                 // self.mode = Mode::List;
    //                 self.task_update = None;
    //             }
    //             KeyCode::Enter => {
    //                 task_update.submit_and_reset();
    //                 self.task_update = None;
    //                 // self.mode = Mode::List;
    //                 self.refresh();
    //             }
    //             _ => task_update.handle_key_events(key_event),
    //         }
    //     }
    // }
    pub fn refresh(&mut self) {
        self.tasks
            .set_items(Task::list(&mut self.conn.borrow_mut()).unwrap());
    }
}

impl Component for TaskList {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        self.tasks.render(f, area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        self.handle_list_key_events(key)
    }
}
