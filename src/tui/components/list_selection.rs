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
    title: &'static str,
}

impl<T> ListSelection<T>
where
    T: Into<ListItem<'static>> + Clone + PartialEq,
{
    pub fn new(items: Vec<T>, title: &'static str) -> Self {
        Self {
            items,
            item_cursor: ListState::default().with_selected(Some(0)),
            active: false,
            title,
        }
    }
    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.item_cursor = ListState::default().with_selected(Some(0));
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
    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            self.item_cursor.select(None);
            return;
        }

        let next_index = self
            .item_cursor
            .selected()
            .map_or(0, |selected| (selected + 1) % self.items.len());

        self.item_cursor.select(Some(next_index));
    }

    pub fn select_previous(&mut self) {
        if self.items.is_empty() {
            self.item_cursor.select(None);
            return;
        }

        let previous_index = self
            .item_cursor
            .selected()
            .map_or(self.items.len() - 1, |selected| {
                if selected == 0 {
                    self.items.len() - 1
                } else {
                    selected - 1
                }
            });

        self.item_cursor.select(Some(previous_index));
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
    T: Into<ListItem<'static>> + Clone + PartialEq,
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
            .block(Block::bordered().title(self.title))
            .highlight_style(highlight_style)
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        f.render_stateful_widget(task_list, area, &mut self.item_cursor);
    }
    fn handle_key_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('j') => self.select_next(),
            KeyCode::Char('k') => self.select_previous(),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::KeyCode;
    use ratatui::crossterm::event::KeyEvent;

    #[derive(Debug, Clone, PartialEq)]
    struct TestItem {
        name: &'static str,
    }

    impl Into<ListItem<'static>> for TestItem {
        fn into(self) -> ListItem<'static> {
            ListItem::new(self.name)
        }
    }

    #[test]
    fn test_new_list_selection() {
        let items = vec![TestItem { name: "Item 1" }, TestItem { name: "Item 2" }];
        let list_selection = ListSelection::new(items.clone(), "Test List");

        assert_eq!(list_selection.items, items);
        assert_eq!(list_selection.title, "Test List");
        assert_eq!(list_selection.item_cursor.selected(), Some(0));
        assert!(!list_selection.active);
    }

    #[test]
    fn test_set_items() {
        let mut list_selection = ListSelection::new(vec![], "Test List");
        let new_items = vec![TestItem { name: "Item 1" }, TestItem { name: "Item 2" }];
        list_selection.set_items(new_items.clone());

        assert_eq!(list_selection.items, new_items);
        assert_eq!(list_selection.item_cursor.selected(), Some(0));
    }

    #[test]
    fn test_switch_active() {
        let items = vec![TestItem { name: "Item 1" }, TestItem { name: "Item 2" }];
        let mut list_selection = ListSelection::new(items, "Test List");
        assert!(!list_selection.active);

        list_selection.switch_active();
        assert!(list_selection.active);

        list_selection.switch_active();
        assert!(!list_selection.active);
    }

    #[test]
    fn test_selected() {
        let items = vec![TestItem { name: "Item 1" }, TestItem { name: "Item 2" }];
        let list_selection = ListSelection::new(items.clone(), "Test List");

        assert_eq!(list_selection.selected(), Some(&items[0]));
    }

    #[test]
    fn test_selected_next() {
        let items = vec![TestItem { name: "Item 1" }];
        let mut list_selection = ListSelection::new(items.clone(), "Test List");

        assert_eq!(list_selection.selected(), Some(&items[0]));
        list_selection.select_next();
        assert_eq!(list_selection.selected(), Some(&items[0]));
    }

    #[test]
    fn test_select_next_empty_list() {
        let mut list_selection: ListSelection<TestItem> = ListSelection::new(vec![], "Test List");

        list_selection.select_next();
        assert_eq!(list_selection.item_cursor.selected(), None);
    }

    #[test]
    fn test_select_previous_empty_list() {
        let mut list_selection: ListSelection<TestItem> = ListSelection::new(vec![], "Test List");

        list_selection.select_previous();
        assert_eq!(list_selection.item_cursor.selected(), None);
    }

    #[test]
    fn test_select_next_boundary() {
        let items = vec![TestItem { name: "Item 1" }, TestItem { name: "Item 2" }];
        let mut list_selection = ListSelection::new(items.clone(), "Test List");

        list_selection.select_next();
        assert_eq!(list_selection.item_cursor.selected(), Some(1));

        list_selection.select_next();
        assert_eq!(list_selection.item_cursor.selected(), Some(0));
    }

    #[test]
    fn test_select_previous_boundary() {
        let items = vec![TestItem { name: "Item 1" }, TestItem { name: "Item 2" }];
        let mut list_selection = ListSelection::new(items.clone(), "Test List");

        list_selection.select_previous();
        assert_eq!(list_selection.item_cursor.selected(), Some(1));

        list_selection.select_previous();
        assert_eq!(list_selection.item_cursor.selected(), Some(0));
    }

    #[test]
    fn test_set_selected() {
        let mut list_selection: ListSelection<TestItem> = ListSelection::new(vec![], "Test List");

        list_selection.select_next();
        assert_eq!(list_selection.item_cursor.selected(), None);
    }

    #[test]
    fn test_handle_key_events() {
        let items = vec![TestItem { name: "Item 1" }, TestItem { name: "Item 2" }];
        let mut list_selection = ListSelection::new(items.clone(), "Test List");

        list_selection.handle_key_events(KeyEvent::from(KeyCode::Char('j')));
        assert_eq!(list_selection.item_cursor.selected(), Some(1));

        list_selection.handle_key_events(KeyEvent::from(KeyCode::Char('k')));
        assert_eq!(list_selection.item_cursor.selected(), Some(0));
    }
}
