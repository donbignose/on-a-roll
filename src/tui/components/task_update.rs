use std::{cell::RefCell, rc::Rc};

use diesel::SqliteConnection;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};

use crate::models::{task_status::TaskStatus, Task};

use super::{
    multi_input::{Inputs, MultiInput},
    Component, InputSubmit,
};

pub struct TaskUpdate {
    inputs: MultiInput<TaskStatus>,
    conn: Rc<RefCell<SqliteConnection>>,
    task_id: i32,
}

impl TaskUpdate {
    pub fn new(
        conn: Rc<RefCell<SqliteConnection>>,
        task_id: i32,
        title: String,
        description: Option<String>,
        status: TaskStatus,
    ) -> Self {
        let mut update = Self {
            conn,
            task_id,
            inputs: MultiInput::new(),
        };
        update.inputs.set_inputs(title, description, status);
        update
    }
}

impl InputSubmit for TaskUpdate {
    fn submit(&self) {
        let Inputs {
            title,
            description,
            status,
        } = self.inputs.get_inputs();

        Task::update(
            &mut self.conn.borrow_mut(),
            self.task_id,
            Some(title),
            Some(description),
            Some(*status),
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
        let block = Block::default()
            .borders(Borders::ALL) // Add borders on all sides
            .title(format!("Task Update for task {}", self.task_id)) // Optional: Add a title to the border
            .style(Style::default().add_modifier(Modifier::BOLD)); // Add styles if needed

        let inner_area = block.inner(area);
        f.render_widget(block, area);
        self.inputs.render(f, inner_area);
    }
    fn handle_key_events(&mut self, key: KeyEvent) {
        self.inputs.handle_key_events(key);
    }
}
