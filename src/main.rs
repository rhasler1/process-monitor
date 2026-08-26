// Internal project imports
use process_monitor::app::{App, AppEventState};
use process_monitor::config::AppConfig;
use process_monitor::events::app_event::{AppEvent, AppEvents};
use process_monitor::adapters::crossterm::input::Key;
use process_monitor::services::config_worker::{ConfigCallerMessage, ConfigWorker, ConfigWorkerMessage};
use process_monitor::services::sysinfo_worker::{SysinfoWorker, CallerMessage, WorkerMessage};
use process_monitor::terminal::{restore_terminal, setup_terminal};

// std library import
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::TrySendError::{Disconnected, Full};

use anyhow::anyhow;

// log
use log::{debug, info, warn, error};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Logger initialized");

    // Setup config worker thread::START
    let config_worker = ConfigWorker::default();
    
    // Build config directory (if it does not already exist)
    // The channel is empty at this point, the blocking call
    // should not block.
    config_worker
        .send(ConfigCallerMessage::BuildConfigDir)
        .map_err(|e| anyhow!(e))?;

    // Wait for worker thread to finish work
    let msg = config_worker
        .next()
        .map_err(|e| anyhow!(e))?;

    // Check message for error
    if matches!(msg, ConfigWorkerMessage::Error(_e)) {
        return Err(anyhow!("Config worker message (fix this error reporting)")).into();
    }
    // Setup config worker thread::END

    // Setup AppConfig::BEGIN
    // Send read config message
    config_worker
        .send(ConfigCallerMessage::ReadConfig)
        .map_err(|e| anyhow!(e))?;

    let msg = config_worker
        .next()
        .map_err(|e| anyhow!(e))?;

    let serialized_config = match msg {
        ConfigWorkerMessage::DoneReadingConfig(s) => {
            s
        }
        _ => return Err(anyhow!("Unexpected message"))
    };

    let app_config: AppConfig = toml::from_str(
        &serialized_config
    ).unwrap_or_default();
    // Setup AppConfig::END

    // Setup App::BEGIN
    // Transfer ownership of config; events should call app.config() to get refresh rate
    let mut app = match App::new_with_config(app_config) {
        Ok(app) => app,
        Err(e) => {
            error!("Error upon app creation: {e}");
            return Err(anyhow!(e));
        }
    };
    // Setup App::END

    // Setup system info worker::BEGIN
    let sysinfo_worker = SysinfoWorker::default();

    sysinfo_worker
        .send(CallerMessage::BuildProcessSnapShot)
        .map_err(|e| anyhow!(e))?;

    let msg = sysinfo_worker
        .next()
        .map_err(|e| anyhow!(e))?;

    let process_snapshot = match msg {
        WorkerMessage::Done(snapshot) => {
            snapshot
        }
        _ => {
            return Err(
                anyhow!("Unexpected message from sysinfo worker")
            )
        }
    };

    // Update app with snapshot
    match app.model_update(process_snapshot) {
        Ok(_) => {}
        Err(e) => {
            restore_terminal()?;
            return Err(anyhow!("{e}"))
        }
    }
    
    // Create AppEvents MPSC channel
    // TODO: AppEvents::new_with_config();
    //       AppEvents main event loop needs to be
    //       refactored to support this.
    let app_events = AppEvents::default();

    // Terminal setup
    let mut terminal = setup_terminal()
        .inspect_err(|e| {
            error!("Terminal setup error: {:?}", e)
        })?;

    info!("Terminal setup complete");
    
    // Main event loop
    info!("Entering main event loop");
    loop {
        // Check worker queue for messages
        match sysinfo_worker.try_next() {
            Ok(WorkerMessage::Done(process_snapshot)) => {
                debug!("`Main` received Done(process_snapshot) message");
                match app.model_update(process_snapshot) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("{e}");
                        break;
                    }
                }
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
                    error!("error: {}", err);
                }
            }
        })?;
        match app_events.next()? {
            AppEvent::Key(key) => {
                match app.key_event(key)? {
                    AppEventState::TerminatePid(pid) => {
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
                    AppEventState::NotConsumed => {
                        if matches!(key, Key::Ctrlc) {
                            info!("`Main`: event to exit app...");
                            break;
                        }
                    }
                    _ => {}
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

    restore_terminal()?;
    info!("Terminal restore complete");
    info!("End of main");
    Ok(())
}
