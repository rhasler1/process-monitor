/*
TODOS:
1. [3/12/26] The ProcessComponent's TableModel should be able to save state between runs. For example, a column configuration should be savable.
This means the app must attempt to read from a config file on start. Initial idea is to add a config.toml file. This also means that configurable
component's model should have a constructor that accepts config.
*/

// Internal project imports
use process_monitor::app::App;
use process_monitor::events::app_event::{AppEvent, AppEvents};
use process_monitor::adapters::crossterm::input::Key;
use process_monitor::services::sysinfo_worker::{SysinfoWorker, CallerMessage, WorkerMessage};
// std library import
use std::sync::mpsc;
use std::sync::mpsc::TrySendError::{Disconnected, Full};
// log
use log::{info, warn, error};


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
    if sysinfo_worker.send(CallerMessage::BuildProcessSnapShot).is_err() {
        // mpsc channel `caller` => `worker` disconnected; exit program
        tear_down()?;
        std::process::exit(1);
    }
    // Using blocking next to wait for first domain model to be built before entering event loop
    match sysinfo_worker.next()? {
        WorkerMessage::Done(process_snapshot) => {
            info!("`Main` received domain model from `worker`");
            app.model_update(process_snapshot);
        } // mpsc channel channel `worker` => `caller` disconnected; exit program
        _ => {
            error!("The MPSC channel from `worker` to `caller` DISCONNECTED");
            tear_down()?;
            info!("Terminal tear down complete");
            info!("Exiting main");
            std::process::exit(1);
        }
    }
    // Main event loop
    info!("Entering main event loop");
    loop {
        // Get new domain model from `worker`
        match sysinfo_worker.try_next() {
            Ok(WorkerMessage::Done(process_snapshot)) => {
                // update state
                app.model_update(process_snapshot);
            }
            Ok(WorkerMessage::Error(_recv_error)) => {
                warn!("`Main` receiver error");
                break;
            } // No new model; this is not an Error and is expected majority of passes
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                warn!("The MPSC channel from `worker` to `main` DISCONNECTED");
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
        // Match next AppEvent TODO [2/24/26] propagate errors found in `key_event` here and exit
        // gracefully
        match app_events.next()? {
            AppEvent::Key(key) => {
                if !app.key_event(key).is_consumed() && key == Key::Char('q') {
                    break;
                }
            }
            AppEvent::RebuildDomain => {
                match sysinfo_worker.try_send(CallerMessage::BuildProcessSnapShot) {
                    Ok(_)  => continue,
                    Err(try_send_error) => {
                        match try_send_error {
                            Full(_b) => {
                                warn!("MPSC channel from `main` to `worker` is FULL");
                            }
                            Disconnected(_b) => { 
                                warn!("MPSC channel from `main` to `worker` is DISCONNECTED");
                                break; 
                            }
                        }
                    }
                }
            }
            AppEvent::Tick => continue
        }
    }
    tear_down();
    info!("Terminal tear down complete");
    info!("End of main");
    Ok(())
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
