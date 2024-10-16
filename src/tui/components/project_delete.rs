use std::{cell::RefCell, rc::Rc};

use diesel::SqliteConnection;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::models::Project;

use super::{Component, InputSubmit};

pub struct ProjectDelete {
    project_id: i32,
    conn: Rc<RefCell<SqliteConnection>>,
}

impl ProjectDelete {
    pub fn new(conn: Rc<RefCell<SqliteConnection>>, project_id: i32) -> Self {
        Self { conn, project_id }
    }
}

impl InputSubmit for ProjectDelete {
    fn submit(&self) {
        Project::delete(&mut self.conn.borrow_mut(), self.project_id).unwrap();
    }

    fn reset(&mut self) {}
}

impl Component for ProjectDelete {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);
        let text = vec![
            Line::raw("Are you sure you want to delete this project?"),
            Line::raw("This action cannot be undone."),
        ];
        let paragraph = Paragraph::new(text).block(
            Block::default()
                .title(format!("Delete Project {}", self.project_id))
                .borders(Borders::ALL),
        );
        frame.render_widget(paragraph, area);
    }

    fn handle_key_events(&mut self, _key: KeyEvent) {}
}
