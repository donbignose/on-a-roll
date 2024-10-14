use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::models::task_status::TaskStatus;

use super::{list_selection::ListSelection, user_input::UserInput, Component};

pub enum TaskInputField {
    Title,
    Description,
    Status,
}
pub struct MultiInput {
    title: UserInput,
    description: UserInput,
    status: ListSelection<TaskStatus>,
    active_field: TaskInputField,
}

pub struct TaskInputs<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub status: TaskStatus,
}

impl MultiInput {
    pub fn new() -> Self {
        Self {
            title: UserInput::new("Task Title".to_string(), true),
            description: UserInput::new("Task Description".to_string(), false),
            active_field: TaskInputField::Title,
            status: ListSelection::new(vec![TaskStatus::Todo, TaskStatus::InProgress]),
        }
    }
    fn switch_field(&mut self) {
        match self.active_field {
            TaskInputField::Title => {
                self.active_field = TaskInputField::Description;
                self.description.switch_active();
                self.title.switch_active();
            }
            TaskInputField::Description => {
                self.active_field = TaskInputField::Status;
                self.description.switch_active();
                self.status.switch_active();
            }
            TaskInputField::Status => {
                self.active_field = TaskInputField::Title;
                self.status.switch_active();
                self.title.switch_active();
            }
        };
    }

    pub fn get_inputs(&self) -> TaskInputs {
        TaskInputs {
            title: self.title.get_input(),
            description: self.description.get_input(),
            status: self.status.selected().unwrap(),
        }
    }
    pub fn reset(&mut self) {
        self.title.reset();
        self.description.reset();
        self.status.reset();
        self.active_field = TaskInputField::Title;
    }
}

impl Component for MultiInput {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        let [title_area, description_area, status_area] = Layout::vertical([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .areas(area);

        self.title.render(f, title_area);
        self.description.render(f, description_area);
        self.status.render(f, status_area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                self.switch_field();
            }
            _ => match self.active_field {
                TaskInputField::Title => self.title.handle_key_events(key),
                TaskInputField::Description => self.description.handle_key_events(key),
                TaskInputField::Status => self.status.handle_key_events(key),
            },
        }
    }
}
