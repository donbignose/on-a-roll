mod list_selection;
mod multi_input;
pub mod popup;
mod project_delete;
mod project_input;
pub mod project_list;
mod project_update;
mod task_delete;
pub mod task_input;
pub mod task_list;
pub mod task_update;
mod user_input;

use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

pub trait Component {
    fn render(&mut self, f: &mut Frame, area: Rect);
    fn handle_key_events(&mut self, key: KeyEvent);
}

pub trait InputSubmit {
    fn submit_and_reset(&mut self) {
        self.submit();
        self.reset();
    }
    fn submit(&self);
    fn reset(&mut self);
}
