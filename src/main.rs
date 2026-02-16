use process_monitor::app::application::App;
use process_monitor::app::events::event::{AppEvent, AppEvents};
use process_monitor::adapters::crossterm::input::*;

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
    let mut app = App::init();

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
                if !app.key_event(key).is_consumed() && key == KeyInput::Char('q') {
                    break;
                }
            }
            AppEvent::MouseInputEvent(mouse) => {
                let _ = app.mouse_event(mouse);
            }
            AppEvent::Refresh => {
                app.refresh_event();
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
