mod list_selection;
mod multi_input;
pub mod task_input;
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
