mod list_selection;
pub mod task_input;
mod user_input;

use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

pub trait Component {
    fn render(&mut self, f: &mut Frame, area: Rect);
    fn handle_key_events(&mut self, key: KeyEvent);
}
