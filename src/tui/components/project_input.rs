use diesel::SqliteConnection;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::models::{project_status::ProjectStatus, Project};

use super::{
    multi_input::{Inputs, MultiInput},
    Component, InputSubmit,
};

pub struct ProjectInput {
    conn: Rc<RefCell<SqliteConnection>>,
    inputs: MultiInput<ProjectStatus>,
}

impl ProjectInput {
    pub fn new(conn: Rc<RefCell<SqliteConnection>>) -> Self {
        Self {
            conn,
            inputs: MultiInput::new(),
        }
    }
}

impl InputSubmit for ProjectInput {
    fn submit(&self) {
        let Inputs {
            title,
            description,
            status,
        } = self.inputs.get_inputs();
        Project::create(
            &mut self.conn.borrow_mut(),
            Some(title),
            Some(description),
            Some(*status),
        )
        .unwrap();
    }

    fn reset(&mut self) {
        self.inputs.reset();
    }
}

impl Component for ProjectInput {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL) // Add borders on all sides
            .title("Project Creation") // Optional: Add a title to the border
            .style(Style::default().add_modifier(Modifier::BOLD)); // Add styles if needed

        let inner_area = block.inner(area);
        f.render_widget(block, area);
        self.inputs.render(f, inner_area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        self.inputs.handle_key_events(key);
    }
}
