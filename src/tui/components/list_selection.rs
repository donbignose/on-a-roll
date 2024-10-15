use derive_setters::Setters;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Clear, List, ListItem, ListState},
    Frame,
};

use super::Component;

#[derive(Debug, Setters)]
pub struct ListSelection<T>
where
    T: Into<ListItem<'static>> + Clone,
{
    items: Vec<T>,
    item_cursor: ListState,
    active: bool,
}

impl<T> ListSelection<T>
where
    T: Into<ListItem<'static>> + Clone + PartialEq,
{
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            item_cursor: ListState::default().with_selected(Some(0)),
            active: false,
        }
    }
    pub fn switch_active(&mut self) {
        self.active = !self.active
    }

    pub fn selected(&self) -> Option<&T> {
        self.item_cursor
            .selected()
            .map(|selected| &self.items[selected])
    }
    pub fn reset(&mut self) {
        self.item_cursor.select(Some(0));
    }
    pub fn set_selected(&mut self, object: T) {
        if let Some(index) = self.items.iter().position(|item| *item == object) {
            self.item_cursor.select(Some(index));
        } else {
            eprintln!("Item not found in the list.");
        }
    }
}

impl<T> Component for ListSelection<T>
where
    T: Into<ListItem<'static>> + Clone,
{
    fn render(&mut self, f: &mut Frame, area: Rect) {
        f.render_widget(Clear, area);
        let highlight_style = if self.active {
            Style::default()
                .bg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC)
        } else {
            Style::default().add_modifier(Modifier::ITALIC) // Keep the italic style when not active
        };
        let task_list = List::new(self.items.clone())
            .block(Block::bordered().title("Tasks"))
            .highlight_style(highlight_style)
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        f.render_stateful_widget(task_list, area, &mut self.item_cursor);
    }
    fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('j') => self.item_cursor.select_next(),
            KeyCode::Char('k') => self.item_cursor.select_previous(),
            _ => {}
        }
    }
}
