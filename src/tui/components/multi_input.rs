use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    widgets::ListItem,
    Frame,
};
use strum::IntoEnumIterator;

use super::{list_selection::ListSelection, user_input::UserInput, Component};
use strum::EnumIter;

#[derive(Debug, Clone, PartialEq, EnumIter)]
pub enum InputField {
    Title,
    Description,
    Status,
}
pub struct MultiInput<T>
where
    T: Into<ListItem<'static>> + Clone,
{
    title: UserInput,
    description: UserInput,
    status: ListSelection<T>,
    active_field: InputField,
}

pub struct Inputs<'a, T> {
    pub title: &'a str,
    pub description: &'a str,
    pub status: &'a T,
}

impl<T> MultiInput<T>
where
    T: Into<ListItem<'static>> + Clone + PartialEq + IntoEnumIterator,
{
    pub fn new() -> Self {
        Self {
            title: UserInput::new("Task Title".to_string(), true),
            description: UserInput::new("Task Description".to_string(), false),
            active_field: InputField::Title,
            status: ListSelection::new(T::iter().collect()),
        }
    }
    fn switch_field(&mut self, reverse: bool) {
        let fields: Vec<InputField> = InputField::iter().collect();
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
            InputField::Title => self.title.switch_active(),
            InputField::Description => self.description.switch_active(),
            InputField::Status => self.status.switch_active(),
        }

        // Update the active field
        self.active_field = fields[index].clone();

        // Activate the new field
        match self.active_field {
            InputField::Title => self.title.switch_active(),
            InputField::Description => self.description.switch_active(),
            InputField::Status => self.status.switch_active(),
        };
    }

    pub fn get_inputs(&self) -> Inputs<T> {
        Inputs {
            title: self.title.get_input(),
            description: self.description.get_input(),
            status: self.status.selected().unwrap(),
        }
    }
    pub fn reset(&mut self) {
        self.title.reset();
        self.description.reset();
        self.status.reset();
        self.active_field = InputField::Title;
    }

    pub fn set_inputs(&mut self, title: String, description: Option<String>, status: T) {
        self.title.set_input(title);
        if let Some(description) = description {
            self.description.set_input(description);
        }
        self.status.set_selected(status)
    }
}

impl<T> Component for MultiInput<T>
where
    T: Into<ListItem<'static>> + Clone + PartialEq + IntoEnumIterator,
{
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
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.switch_field(true); // Switch in reverse order when Shift+Tab is pressed
                } else {
                    self.switch_field(false); // Switch forward when Tab is pressed
                }
            }
            _ => match self.active_field {
                InputField::Title => self.title.handle_key_events(key),
                InputField::Description => self.description.handle_key_events(key),
                InputField::Status => self.status.handle_key_events(key),
            },
        }
    }
}
