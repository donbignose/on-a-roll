use std::{cell::RefCell, rc::Rc};

use diesel::SqliteConnection;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders},
    Frame,
};

use crate::models::{project_status::ProjectStatus, Project};

use super::{
    multi_input::{Inputs, MultiInput},
    Component, InputSubmit,
};

pub struct ProjectUpdate {
    inputs: MultiInput<ProjectStatus>,
    conn: Rc<RefCell<SqliteConnection>>,
    project_id: i32,
}

impl ProjectUpdate {
    pub fn new(
        conn: Rc<RefCell<SqliteConnection>>,
        project_id: i32,
        title: String,
        description: Option<String>,
        status: ProjectStatus,
    ) -> Self {
        let mut update = Self {
            conn,
            project_id,
            inputs: MultiInput::new(),
        };
        update.inputs.set_inputs(title, description, status);
        update
    }
    pub fn from_project(conn: Rc<RefCell<SqliteConnection>>, project: &Project) -> Self {
        Self::new(
            conn,
            project.id,
            project.title.clone(),
            project.description.clone(),
            project.status,
        )
    }
}

impl InputSubmit for ProjectUpdate {
    fn submit(&self) {
        let Inputs {
            title,
            description,
            status,
        } = self.inputs.get_inputs();

        Project::update(
            &mut self.conn.borrow_mut(),
            self.project_id,
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

impl Component for ProjectUpdate {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL) // Add borders on all sides
            .title(format!("Project Update for project {}", self.project_id)) // Optional: Add a title to the border
            .style(Style::default().add_modifier(Modifier::BOLD)); // Add styles if needed

        let inner_area = block.inner(area);
        f.render_widget(block, area);
        self.inputs.render(f, inner_area);
    }
    fn handle_key_events(&mut self, key: KeyEvent) {
        self.inputs.handle_key_events(key);
    }
}
