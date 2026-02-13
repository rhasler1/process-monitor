use rust_tui_template::app::application::App;
use rust_tui_template::app::event::{AppEvent, AppEvents};
use rust_tui_template::adapters::crossterm::input::*;

fn main() -> anyhow::Result<()> {
    // Terminal setup
    let backend = ratatui::backend::CrosstermBackend::new(
        std::io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture)?;
    terminal.clear()?;

    // Create App
    let mut app = App::default();

    // Create AppEvents MPSC channel
    let app_events = AppEvents::default();

    // Main event loop
    loop {
        // Draw app
        terminal.draw(|frame| {
            match app.draw(frame) {
                Ok(_state) => {}
                Err(err) => {
                    println!("error: {}", err.to_string());
                }
            }
        })?;

        // Get next AppEvent and match
        match app_events.next()? {
            AppEvent::KeyInputEvent(key) => {
                if key == KeyInput::Char('q') {
                    break;
                }
            }
            AppEvent::MouseInputEvent(mouse) => {
                if mouse.kind == MouseInputKind::LeftClick {
                    break;
                }
            }
            AppEvent::Tick => continue
        }
    }

    // Terminal tear down
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
        )?;

    Ok(())
}
