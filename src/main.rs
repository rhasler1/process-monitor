// Internal project imports
use process_monitor::app::App;
use process_monitor::events::app_event::{AppEvent, AppEvents};
use process_monitor::adapters::crossterm::input::Key;
use process_monitor::services::sysinfo_worker::{SysinfoWorker, CallerMessage, WorkerMessage};
// std library import
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::TrySendError::{Disconnected, Full};
// log
use log::{debug, info, warn, error};


fn main() -> anyhow::Result<()> {
    // Logger init
    env_logger::init();
    info!("Logger initialized");
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
    info!("Terminal setup complete");

    // Create App
    let mut app = App::default();
    // Create AppEvents MPSC channel
    let app_events = AppEvents::default();
    // Create sysinfo worker
    let sysinfo_worker = SysinfoWorker::default();
    // Send build message to worker
    debug!("`Main`: sending initial BuildProcessSnapShot message");
    if sysinfo_worker.send(CallerMessage::BuildProcessSnapShot).is_err() {
        // mpsc channel `caller` => `worker` disconnected; exit program
        tear_down()?;
        std::process::exit(1);
    }
    // Using blocking next to wait for first domain model to be built before entering event loop
    match sysinfo_worker.next()? {
        WorkerMessage::Done(process_snapshot) => {
            debug!("`Main` received domain model from `worker`");
            app.model_update(process_snapshot);
        }
        WorkerMessage::DoneTerminateProcess => {
            error!("`Main`: received DoneTerminateProcess message before main event loop, exiting program...");
            tear_down()?;
            info!("Terminal tear down complete");
            info!("Exiting main");
            std::process::exit(1);
        }
    }
    // Main event loop
    info!("Entering main event loop");
    loop {
        // Check worker queue for messages
        match sysinfo_worker.try_next() {
            Ok(WorkerMessage::Done(process_snapshot)) => {
                debug!("`Main` received Done(process_snapshot) message");
                app.model_update(process_snapshot);
            }
            Ok(WorkerMessage::DoneTerminateProcess) => {
                debug!("`Main` received DoneTerminateProcess message");
            }
            Err(TryRecvError::Empty) => {
                debug!("`Main`: mpsc channel is empty");
            }
            Err(TryRecvError::Disconnected) => {
                error!("The MPSC channel from `worker` to `main` DISCONNECTED, exiting program...");
                break;
            }
        }
        // Draw app
        terminal.draw(|frame| {
            match app.draw(frame) {
                Ok(_state) => {}
                Err(err) => {
                    error!("error: {}", err.to_string());
                }
            }
        })?;
        match app_events.next()? {
            AppEvent::Key(key) => {
                let event_state = app.key_event(key);
                if event_state.is_return_payload() {
                    // safe to unwrap here
                    let pid = event_state.payload().unwrap();
                    match sysinfo_worker.try_send(CallerMessage::TerminateProcess(pid)) {
                        Ok(_) => { continue }
                        Err(try_send_err) => {
                            match try_send_err {
                                Full(_) => {
                                    warn!("MPSC channel from `main` to `worker` is FULL");
                                }
                                Disconnected(_) => {
                                    error!("MPSC channel from `main` to `worker` is DISCONNECTED");
                                break;
                                }
                            }

                        }
                    }
                }
                if !event_state.is_consumed() && key == Key::Esc {
                    info!("`Main`: event to exit app...");
                    break;
                }
            }
            AppEvent::RebuildDomain => {
                match sysinfo_worker.try_send(CallerMessage::BuildProcessSnapShot) {
                    Ok(_)  => {
                        debug!("`Main` try_send(BuildMessage) successful");
                    }
                    Err(try_send_error) => {
                        match try_send_error {
                            Full(_b) => {
                                warn!("MPSC channel from `main` to `worker` is FULL");
                            }
                            Disconnected(_b) => { 
                                error!("MPSC channel from `main` to `worker` is DISCONNECTED");
                                break; 
                            }
                        }
                    }
                }
            }
            AppEvent::Tick => continue
        }
    }

    tear_down()?;
    info!("Terminal tear down complete");
    info!("End of main");
    Ok(())
}

// Terminal tear down
fn tear_down() -> anyhow::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
        )?;
    Ok(())
}

/*
TODO:
- Clean up process_table naming convention
- Write integration tests at App level
- Write key_config and inject into components (remove hardcoded key values from controllers)
- Serialize & deserialize component structures so that column order and sort order can be saved across runs
- Improve termination view
- Rewrite main README.md
*/