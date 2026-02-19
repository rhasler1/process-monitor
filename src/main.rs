use process_monitor::app::App;
use process_monitor::events::app_event::{AppEvent, AppEvents};
use process_monitor::adapters::crossterm::input::*;
use process_monitor::services::sysinfo_worker::{SysinfoWorker,CallerMessage,WorkerMessage};

use std::sync::mpsc;

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
    // Create sysinfo worker
    let sysinfo_worker = SysinfoWorker::default();
    // Send build message to worker
    if sysinfo_worker.send(CallerMessage::BuildProcessSnapShot).is_err() {
        // mpsc channel `caller` => `worker` disconnected; exit program
        tear_down();
        std::process::exit(1);
    }
    // Using blocking next to wait for first domain model to be built before entering event loop
    match sysinfo_worker.next()? {
        WorkerMessage::Done(process_snapshot) => {
            app.update(process_snapshot);
        } // mpsc channel channel `worker` => `caller` disconnected; exit program
        _ => {
            tear_down();
            std::process::exit(1);
        }
    }
    // Main event loop
    loop {
        // Get new domain model from `worker`
        match sysinfo_worker.try_next() {
            Ok(WorkerMessage::Done(process_snapshot)) => {
                // update state
                app.update(process_snapshot);
            }
            Ok(WorkerMessage::Error(recv_error)) => {
                break;
            } /* No new model; this is not an Error and is expected majority of checks*/
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                break;
            }
        }
        // Draw app
        terminal.draw(|frame| {
            match app.draw(frame) {
                Ok(_state) => {}
                Err(err) => {
                    println!("error: {}", err.to_string());
                }
            }
        })?;
        // Match next AppEvent
        match app_events.next()? {
            AppEvent::KeyInputEvent(key) => {
                if !app.key_event(key).is_consumed() && key == KeyInput::Char('q') {
                    break;
                }
            }
            AppEvent::MouseInputEvent(mouse) => {
                //let _ = app.mouse_event(mouse);
            }
            AppEvent::Refresh => {
                // Send `build` message to worker
                if sysinfo_worker.send(CallerMessage::BuildProcessSnapShot).is_err() {
                    break;
                }
            }
            AppEvent::Tick => continue
        }
    }
    tear_down()
}

fn tear_down() -> anyhow::Result<()> {
    // Terminal tear down
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
        )?;
    Ok(())
}
