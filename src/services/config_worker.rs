// TODO: Write unit tests
use anyhow::{anyhow, Result};
use log::{debug, warn, error};
use directories::ProjectDirs;

use std::fs;
use std::thread;
use std::sync::mpsc::{
    Receiver, TryRecvError, RecvError,
    SyncSender, TrySendError, SendError,
    sync_channel
};

const AUTHOR:        &str = "rhasler1";
const DOMAIN:        &str = "io";
const SOFTWARE_NAME: &str = "process-monitor";
const FILE_NAME:     &str = "process_monitor.toml";

/// Message from caller to worker
pub enum ConfigCallerMessage {
    BuildConfigDir,
    WriteConfig(String),
    ReadConfig,
}

/// Message from worker to caller
pub enum ConfigWorkerMessage {
    DoneBuildingConfigDir,
    DoneWritingConfig,
    DoneReadingConfig(String),
    Error(anyhow::Error)
}

/// This structure is for the `main` event loop.
pub struct ConfigWorker {
    rx: Receiver<ConfigWorkerMessage>,
    tx: SyncSender<ConfigCallerMessage>
}

impl Default for ConfigWorker {
    fn default() -> Self {
        const DEFAULT_CHANNEL_CAPACITY: usize = 2;

        // This channel is used for communication from `main` thread to `worker` thread
        let (to_caller_tx, from_worker_rx) = sync_channel(DEFAULT_CHANNEL_CAPACITY);
        
        // This channel is used for communication from `worker` thread to `main` thread 
        let (to_worker_tx, from_caller_rx) = sync_channel(DEFAULT_CHANNEL_CAPACITY);

        thread::spawn(move || -> Result<()> {
            Self::event_loop(&from_caller_rx, &to_caller_tx)
        });

        Self { rx: from_worker_rx, tx: to_worker_tx }
    }
}

impl ConfigWorker {
    /// `try_next` is non-blocking
    //
    /// # Returns
    /// - Ok(ConfigWorkerMessage)
    /// - Err(TryRecvError::Empty)
    /// - Err(TryRecvError::Disconnected)
    pub fn try_next(&self) -> Result<ConfigWorkerMessage, TryRecvError> {
        self.rx.try_recv()
    }

    /// `next` is blocking
    ///
    /// # Returns
    /// - Ok(ConfigWorkerMessage)
    /// - Err if the receiver is disconnected
    pub fn next(&self) -> Result<ConfigWorkerMessage, RecvError> {
        self.rx.recv()
    }

    /// `try_send` is non-blocking
    ///
    /// # Returns
    /// - Ok()
    /// - Err(TrySendErr::Full)
    /// - Err(TrySendErr::Disconnected)
    pub fn try_send(
        &self,
        msg: ConfigCallerMessage
    ) -> Result<(), TrySendError<ConfigCallerMessage>> {
        self.tx.try_send(msg)
    }

    /// `send` is blocking
    ///
    /// # Returns
    /// Error if the receiver is disconnected
    pub fn send(
        &self,
        msg: ConfigCallerMessage
    ) -> Result<(), SendError<ConfigCallerMessage>> {
        self.tx.send(msg)
    }

    /// Config worker event loop
    ///
    /// # Returns
    /// - Err if the channel disconnects
    fn event_loop(
        from_caller_rx: &Receiver<ConfigCallerMessage>,
        to_caller_tx:   &SyncSender<ConfigWorkerMessage>
    ) -> Result<()> {
        loop {
            match from_caller_rx.recv() {
                Ok(ConfigCallerMessage::BuildConfigDir) => {
                    match build_proj_dir() {
                        Ok(_) => {
                            if !send_worker_message(
                                to_caller_tx,
                                ConfigWorkerMessage::DoneBuildingConfigDir
                            ) {
                                break;
                            }
                        }
                        Err(e) => {
                            if !send_worker_message(
                                to_caller_tx,
                                ConfigWorkerMessage::Error(e)
                            ) {
                                break;
                            }
                        }
                    }
                }
                Ok(ConfigCallerMessage::WriteConfig(s)) => {
                    match write_config(s) {
                        Ok(_) => {
                            if !send_worker_message(
                                to_caller_tx,
                                ConfigWorkerMessage::DoneWritingConfig
                            ) {
                                break;
                            }
                        }
                        Err(e) => {
                            if !send_worker_message(
                                to_caller_tx,
                                ConfigWorkerMessage::Error(e)
                            ) {
                                break;
                            }
                        }
                    }
                }
                Ok(ConfigCallerMessage::ReadConfig) => {
                    match read_config() {
                        Ok(s) => {
                            if !send_worker_message(
                                to_caller_tx,
                                ConfigWorkerMessage::DoneReadingConfig(s)
                            ) {
                                break;
                            }
                        }
                        Err(e) => {
                            if !send_worker_message(
                                to_caller_tx,
                                ConfigWorkerMessage::Error(e)
                            ) {
                                break;
                            }
                        }
                    }
                }
                Err(_recv_err) => {
                    break;
                }
            }
        }

        Err(anyhow!("channel disconnected"))
    }
}

/// Sends `msg` using `tx`
///
/// # Behavior
/// - Blocking
///
/// # Returns
/// - True if the `msg` is successfully sent
/// - False if the receiver is disconnected
fn send_worker_message(
    tx: &SyncSender<ConfigWorkerMessage>,
    msg: ConfigWorkerMessage
) -> bool {
    match tx.send(msg) {
        Ok(_) => true,
        Err(SendError(_e)) => {
            error!("receiver is disconnected");
            false
        }
    }
}

/// Ensures the project's configuration directory exists.
///
/// The configuration directory is determined using the platform-specific 
/// configuration directory determined by `ProjectDirs::From` using `DOMAIN`,
/// `AUTHOR`, `SOFTWARE_NAME`.
///
/// If the configuration directory already exists, the function returns successfully.
/// Otherwise, it creates the directory and any required parent directories using
/// `fs::create_dir_all`.
///
/// # Errors
/// Returns an error if:
/// - A valid project directory cannot be determined.
/// - The existence of the configuration directory cannot be verified.
/// - The configuration directory or any required parent directories
///   cannot be created.
fn build_proj_dir() -> Result<()> {
    if let Some(proj_dir) = ProjectDirs::from(DOMAIN, AUTHOR, SOFTWARE_NAME) {
        let config_dir = proj_dir.config_dir();
        
        match fs::exists(config_dir) {
            Ok(true) => {
                debug!("config directory exists: {config_dir:?}");

                Ok(())
            }
            Ok(false) => {
                debug!("config directory does not exist: {config_dir:?}");
                
                match fs::create_dir_all(config_dir) {
                    Ok(_) => {
                        debug!("config directory built at: {config_dir:?}");

                        Ok(())
                    }
                    Err(e) => {
                        warn!("failed to build config directory at: {config_dir:?}");
                        
                        Err(e.into())
                    }
                }
            }
            Err(e) => {
                warn!("config directory's existence cannot be verified");

                Err(e.into())
            }
        }

    } else {
        warn!("valid $HOME path cannot be found");
        
        Err(anyhow!("$HOME directory not found"))
    }
}

// Read the entire contents of the configuration file
// Returns Some(String)
//
// Returns error if file does not already exist

/// Reads the etnire contents of the configuration file.
///
/// The configuration file is located using the platform-
/// specific directory returned by `ProjectDirs::from`,
/// combined with `FILE_NAME`.
///
/// # Returns
/// - `Ok(Some(content))` when the configuration file exists
///   and is a regular file.
/// - `Ok(None)` when the
///   project configuration file does not exist or is not
///   a regular file.
///
/// # Errors
/// Returns an error if the configuration file exists but cannot
/// be read as a UTF-8 string.
fn read_config() -> Result<String> {
    if let Some(proj_dir) = ProjectDirs::from(DOMAIN, AUTHOR, SOFTWARE_NAME) {
        let config_dir = proj_dir.config_dir();
        
        let abs_path = config_dir.join(FILE_NAME);
        
        debug!("reading configuration file: {abs_path:?}");
        
        if abs_path.is_file() {
            let content = fs::read_to_string(&abs_path)?;
            
            return Ok(content)
        } 
    }

    Err(anyhow!("$HOME directory not found"))
} 


// Replace the entire contents of the configuration file

/// Replaces the entire contents of the configuration file with the
/// specified content.
///
/// The configuration file is located in the platform-specific 
/// configuration directory determined by `ProjectDirs::From` using
/// `DOMAIN`, `AUTHOR`, `SOFTWARE_NAME`, `FILE_NAME`.
///
/// If the configuration file does not already exist, it is created.
///
/// # Errors
/// Returns an error if the configuration file cannot be written.
/// - Propagates error from fs::write.
fn write_config(content: String) -> Result<()> {
    if let Some(proj_dir) = ProjectDirs::from(DOMAIN, AUTHOR, SOFTWARE_NAME) {
        let config_dir = proj_dir.config_dir();
        
        let abs_path = config_dir.join(FILE_NAME);

        debug!("writing content to: {abs_path:?}");
        // fs::write() creates the file if it does not already exist
        fs::write(abs_path, &content)?;

        Ok(())
    } else {
        Err(
            anyhow!("$HOME directory not found")
        )
    }
}

