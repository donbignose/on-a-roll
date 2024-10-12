use std::io;

use diesel::SqliteConnection;
use on_a_roll::db::connection::establish_connection;
use on_a_roll::models::Task;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    style::{Modifier, Style},
    widgets::{Block, List, ListState},
    DefaultTerminal, Frame,
};

pub struct App {
    conn: SqliteConnection,
    tasks: Vec<Task>,
    task_state: ListState,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut conn = establish_connection();
        let tasks = Task::list(&mut conn).unwrap();
        Self {
            conn,
            tasks,
            task_state: ListState::default().with_selected(Some(0)),
            exit: false,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&mut self, frame: &mut Frame) {
        let task_list = List::new(&self.tasks)
            .block(Block::bordered().title("Tasks"))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        frame.render_stateful_widget(task_list, frame.area(), &mut self.task_state);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => self.refresh_tasks(),
            KeyCode::Char('j') => self.task_state.select_next(),
            KeyCode::Char('k') => self.task_state.select_previous(),
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
    fn refresh_tasks(&mut self) {
        self.tasks = Task::list(&mut self.conn).unwrap();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
