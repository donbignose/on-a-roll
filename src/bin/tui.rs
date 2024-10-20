use on_a_roll::tui::app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
