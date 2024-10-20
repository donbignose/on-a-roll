use std::{cell::RefCell, rc::Rc};

use diesel::SqliteConnection;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::models::Task;

use super::{Component, InputSubmit};

pub struct TaskDelete {
    task_id: i32,
    conn: Rc<RefCell<SqliteConnection>>,
}

impl TaskDelete {
    pub fn new(conn: Rc<RefCell<SqliteConnection>>, task_id: i32) -> Self {
        Self { conn, task_id }
    }
}

impl InputSubmit for TaskDelete {
    fn submit(&self) {
        Task::delete(&mut self.conn.borrow_mut(), self.task_id).unwrap();
    }

    fn reset(&mut self) {}
}

impl Component for TaskDelete {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);
        let text = vec![
            Line::raw("Are you sure you want to delete this task?"),
            Line::raw("This action cannot be undone."),
        ];
        let paragraph = Paragraph::new(text).block(
            Block::default()
                .title(format!("Delete Task {}", self.task_id))
                .borders(Borders::ALL),
        );
        frame.render_widget(paragraph, area);
    }

    fn handle_key_events(&mut self, _key: KeyEvent) {}
}
