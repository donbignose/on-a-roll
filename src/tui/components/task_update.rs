use std::{cell::RefCell, rc::Rc};

use diesel::SqliteConnection;
use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::models::Task;

use super::{
    multi_input::{MultiInput, TaskInputs},
    Component, InputSubmit,
};

pub struct TaskUpdate {
    inputs: MultiInput,
    conn: Rc<RefCell<SqliteConnection>>,
    task_id: i32,
}

impl TaskUpdate {
    pub fn new(
        conn: Rc<RefCell<SqliteConnection>>,
        task_id: i32,
        title: String,
        description: Option<String>,
    ) -> Self {
        let mut update = Self {
            conn,
            task_id,
            inputs: MultiInput::new(),
        };
        update.inputs.set_inputs(title, description);
        update
    }
}

impl InputSubmit for TaskUpdate {
    fn submit(&self) {
        let TaskInputs {
            title,
            description,
            status,
        } = self.inputs.get_inputs();

        Task::update(
            &mut self.conn.borrow_mut(),
            self.task_id,
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

impl Component for TaskUpdate {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        self.inputs.render(f, area);
    }
    fn handle_key_events(&mut self, key: KeyEvent) {
        self.inputs.handle_key_events(key);
    }
}
