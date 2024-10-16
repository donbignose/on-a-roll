use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use super::{
    project_input::ProjectInput, project_update::ProjectUpdate, task_input::TaskInput,
    task_update::TaskUpdate, Component, InputSubmit,
};

pub enum Popup {
    TaskInput(TaskInput),
    TaskUpdate(TaskUpdate),
    ProjectInput(ProjectInput),
    ProjectUpdate(ProjectUpdate),
}

impl Popup {
    pub fn submit(&mut self) {
        match self {
            Self::TaskInput(task_input) => task_input.submit_and_reset(),
            Self::TaskUpdate(task_update) => task_update.submit_and_reset(),
            Self::ProjectInput(project_input) => project_input.submit_and_reset(),
            Self::ProjectUpdate(project_update) => project_update.submit_and_reset(),
        }
    }
}

impl Component for Popup {
    fn render(&mut self, f: &mut Frame, area: Rect) {
        match self {
            Self::TaskInput(task_input) => task_input.render(f, area),
            Self::TaskUpdate(task_update) => task_update.render(f, area),
            Self::ProjectInput(project_input) => project_input.render(f, area),
            Self::ProjectUpdate(project_update) => project_update.render(f, area),
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) {
        {
            let this = &mut *self;
            match this {
                Popup::TaskInput(task_input) => task_input.handle_key_events(key),
                Popup::TaskUpdate(task_update) => task_update.handle_key_events(key),
                Popup::ProjectInput(project_input) => project_input.handle_key_events(key),
                Popup::ProjectUpdate(project_update) => project_update.handle_key_events(key),
            }
        }
    }
}
