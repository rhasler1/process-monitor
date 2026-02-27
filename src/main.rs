// Internal project imports
use process_monitor::app::App;
use process_monitor::events::app_event::{AppEvent, AppEvents};
use process_monitor::adapters::crossterm::input::{Key};
use process_monitor::services::sysinfo_worker::{SysinfoWorker,CallerMessage,WorkerMessage};
// std library import
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
        tear_down()?;
        std::process::exit(1);
    }
    // Using blocking next to wait for first domain model to be built before entering event loop
    match sysinfo_worker.next()? {
        WorkerMessage::Done(process_snapshot) => {
            app.model_update(process_snapshot);
        } // mpsc channel channel `worker` => `caller` disconnected; exit program
        _ => {
            tear_down()?;
            std::process::exit(1);
        }
    }
    // Main event loop
    loop {
        // Get new domain model from `worker`
        match sysinfo_worker.try_next() {
            Ok(WorkerMessage::Done(process_snapshot)) => {
                // update state
                app.model_update(process_snapshot);
            }
            Ok(WorkerMessage::Error(_recv_error)) => {
                break;
            } /* No new model; this is not an Error and is expected majority of checks*/
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                break;
            }
        }
        // This in it self could be a fun project
        // terminal.draw(|frame| ratatui_renderer.draw(frame, app.render()))?;
        
        // Draw app
        terminal.draw(|frame| {
            match app.draw(frame) {
                Ok(_state) => {}
                Err(err) => {
                    println!("error: {}", err.to_string());
                }
            }
        })?;
        // Match next AppEvent TODO [2/24/26] propogate errors found in `key_event` here and exit
        // gracefully
        match app_events.next()? {
            AppEvent::Key(key) => {
                if !app.key_event(key).is_consumed() && key == Key::Char('q') {
                    break;
                }
            }
            AppEvent::RebuildDomain => {
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
