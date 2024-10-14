use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    Frame,
};
use strum::IntoEnumIterator;

use crate::models::task_status::TaskStatus;

use super::{list_selection::ListSelection, user_input::UserInput, Component};
use strum::EnumIter;

#[derive(Debug, Clone, PartialEq, EnumIter)]
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
            status: ListSelection::new(TaskStatus::iter().collect()),
        }
    }
    fn switch_field(&mut self, reverse: bool) {
        let fields: Vec<TaskInputField> = TaskInputField::iter().collect();
        let mut index = fields.iter().position(|f| f == &self.active_field).unwrap();

        // Determine the next field based on the direction
        if reverse {
            index = if index == 0 {
                fields.len() - 1
            } else {
                index - 1
            };
        } else {
            index = (index + 1) % fields.len();
        }

        // Deactivate the currently active field and activate the new one
        match self.active_field {
            TaskInputField::Title => self.title.switch_active(),
            TaskInputField::Description => self.description.switch_active(),
            TaskInputField::Status => self.status.switch_active(),
        }

        // Update the active field
        self.active_field = fields[index].clone();

        // Activate the new field
        match self.active_field {
            TaskInputField::Title => self.title.switch_active(),
            TaskInputField::Description => self.description.switch_active(),
            TaskInputField::Status => self.status.switch_active(),
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

    pub fn set_inputs(&mut self, title: String, description: Option<String>, status: TaskStatus) {
        self.title.set_input(title);
        if let Some(description) = description {
            self.description.set_input(description);
        }
        self.status.set_selected(status)
    }
}

impl Component for MultiInput {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        let [text_area, list_area] =
            Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(35)])
                .areas(area);
        let [title_area, description_area] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(text_area);

        self.title.render(f, title_area);
        self.description.render(f, description_area);
        self.status.render(f, list_area);
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                println!("{:?}", key.modifiers);
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.switch_field(true); // Switch in reverse order when Shift+Tab is pressed
                } else {
                    self.switch_field(false); // Switch forward when Tab is pressed
                }
            }
            _ => match self.active_field {
                TaskInputField::Title => self.title.handle_key_events(key),
                TaskInputField::Description => self.description.handle_key_events(key),
                TaskInputField::Status => self.status.handle_key_events(key),
            },
        }
    }
}
