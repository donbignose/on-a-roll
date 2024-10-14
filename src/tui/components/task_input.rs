use diesel::SqliteConnection;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};
use std::cell::RefCell;
use std::rc::Rc;

use crate::models::Task;

use super::{
    multi_input::{MultiInput, TaskInputs},
    Component, InputSubmit,
};

pub struct TaskInput {
    conn: Rc<RefCell<SqliteConnection>>,
    inputs: MultiInput,
}

impl TaskInput {
    pub fn new(conn: Rc<RefCell<SqliteConnection>>) -> Self {
        Self {
            conn,
            inputs: MultiInput::new(),
        }
    }
}

impl InputSubmit for TaskInput {
    fn submit(&self) {
        let TaskInputs {
            title,
            description,
            status,
        } = self.inputs.get_inputs();
        Task::create(
            &mut self.conn.borrow_mut(),
            Some(title),
            Some(description),
            Some(status),
            None,
        )
        .unwrap();
    }

    fn reset(&mut self) {
        self.inputs.reset();
    }
}

impl Component for TaskInput {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        self.inputs.render(f, area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        self.inputs.handle_key_events(key);
    }
}
