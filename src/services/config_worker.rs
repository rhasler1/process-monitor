use core::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::sync::mpsc::{
    Receiver, TryRecvError, RecvError,
    SyncSender, TrySendError, SendError,
    sync_channel
};

use anyhow::{anyhow, Result};
use log::{debug, warn, error};
use directories::ProjectDirs;

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
    DoneReadingConfig(Option<String>),
    Error(anyhow::Error)
}

impl fmt::Display for ConfigWorkerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoneBuildingConfigDir => write!(f, "Done building configuration directory"),
            Self::DoneWritingConfig => write!(f, "Done writing configuration file"),
            Self::DoneReadingConfig(_) => write!(f, "Done reading configuration file"),
            Self::Error(e) => write!(f, "{e}"),
        }
    }
}

/// This structure is for the `main` event loop.
pub struct ConfigWorker {
    rx: Receiver<ConfigWorkerMessage>,
    tx: SyncSender<ConfigCallerMessage>,
}

impl Default for ConfigWorker {
    fn default() -> Self {
        Self::with_config_dir(proj_config_dir_path())
    }
}

impl ConfigWorker {
    const DEFAULT_CHANNEL_CAPACITY: usize = 2;

    /// Creates a `ConfigWorker` using the specified config directory.
    ///
    /// This is primarily useful for tests, where a temporary directory
    /// can be injected.
    pub fn new(config_dir_path: impl Into<PathBuf>) -> Self {
        Self::with_config_dir(Some(config_dir_path.into()))
    }

    fn with_config_dir(config_dir_path: Option<PathBuf>) -> Self {
        // This channel is used for communication from `main` thread to `worker` thread 
        let (to_caller_tx, from_worker_rx) = sync_channel(Self::DEFAULT_CHANNEL_CAPACITY);
        
        // This channel is used for communication from `worker` thread to `main` thread 
        let (to_worker_tx, from_caller_rx) = sync_channel(Self::DEFAULT_CHANNEL_CAPACITY);

        thread::spawn(move || -> Result<()> {
            Self::event_loop(
                &from_caller_rx,
                &to_caller_tx,
                config_dir_path
            )
        });

        Self {
            rx: from_worker_rx,
            tx: to_worker_tx
        }
    }

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
        from_caller_rx:     &Receiver<ConfigCallerMessage>,
        to_caller_tx:       &SyncSender<ConfigWorkerMessage>,
        config_dir_path:    Option<PathBuf>,
    ) -> Result<()> {
        let Some(config_dir_path) = config_dir_path else {
            return Err(anyhow!("No config directory path"))
        };

        loop {
            match from_caller_rx.recv() {
                Ok(ConfigCallerMessage::BuildConfigDir) => {
                    match build_proj_config_dir(&config_dir_path) {
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
                    match write_config(s, &config_dir_path) {
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
                    match read_config(&config_dir_path) {
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

/// The configuration directory is determined using the platform-specific 
/// configuration directory determined by `ProjectDirs::From` using `DOMAIN`,
/// `AUTHOR`, `SOFTWARE_NAME`.
fn proj_config_dir_path() -> Option<PathBuf> {
    if let Some(proj_dir) = ProjectDirs::from(DOMAIN, AUTHOR, SOFTWARE_NAME) {
        let config_dir = proj_dir.config_dir();
        
        Some(config_dir.to_owned())
    } else {
        warn!("$HOME directory not found");
        None
    }
}

/// Ensures the project's configuration directory exists.
///
/// If the configuration directory already exists, the function returns successfully.
/// Otherwise, it creates the directory and any required parent directories using
/// `fs::create_dir_all`.
///
/// # Errors
/// Returns an error if:
/// - The existence of the configuration directory cannot be verified.
/// - The configuration directory or any required parent directories
///   cannot be created.
fn build_proj_config_dir(config_dir_path: &Path) -> Result<()> {
    match fs::exists(config_dir_path) {
        Ok(true) => {
            debug!("config directory exists: {config_dir_path:?}");

            Ok(())
        }
        Ok(false) => {
            debug!("config directory does not exist: {config_dir_path:?}");
            
            match fs::create_dir_all(config_dir_path) {
                Ok(_) => {
                    debug!("config directory built at: {config_dir_path:?}");

                    Ok(())
                }
                Err(e) => {
                    warn!("failed to build config directory at: {config_dir_path:?}");
                    
                    Err(e.into())
                }
            }
        }
        Err(e) => {
            warn!("config directory's existence cannot be verified");

            Err(e.into())
        }
    }
}

/// Reads the entire contents of the configuration file.
///
/// # Returns
/// - `Ok(Some(content))` when the configuration file exists
///   and is a regular file.
/// - `Ok(None)` when the
///   project configuration file does not exist.
///
/// # Errors
/// - Error if the configuration file exists but cannot
///   be read as a UTF-8 string.
/// - Error if the configuration path is not a regular file.
fn read_config(config_dir_path: &Path) -> Result<Option<String>> {
    let abs_path = config_dir_path.join(FILE_NAME);
    
    debug!("reading configuration file: {abs_path:?}");

    match fs::metadata(&abs_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                let content = fs::read_to_string(&abs_path)?;
                
                Ok(Some(content))
            } else {
                Err(anyhow!(
                    "configuration path is not a regular file"
                ))
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Ok(None)
        }
        Err(e) => Err(e.into())
    } 
}

/// Replaces the entire contents of the configuration file with the
/// specified content.
///
/// If the configuration file or it's parent directories do not already exist,
/// they are created.
///
/// # Errors
/// - Propagates error from fs::write.
fn write_config(content: String, config_dir_path: &Path) -> Result<()> {
    build_proj_config_dir(config_dir_path)?;
    
    let abs_path = config_dir_path.join(FILE_NAME);

    debug!("writing content to: {abs_path:?}");
    // fs::write() creates the file if it does not already exist
    fs::write(abs_path, &content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_worker_message_succeeds_when_recv_is_connected() {
        let (tx, rx) = sync_channel(1);

        let result = send_worker_message(
            &tx,
            ConfigWorkerMessage::DoneWritingConfig
        );

        assert!(result);

        let received = rx.recv().unwrap();

        assert!(
            matches!(
                received,
                ConfigWorkerMessage::DoneWritingConfig
            )
        );
    }
}
