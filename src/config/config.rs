use anyhow::Result;
use log::{debug, warn};
use std::{fs, io::ErrorKind};
use directories::ProjectDirs;

const FILE_NAME: &str = "process_monitor.toml";
const SOFTWARE_NAME: &str = "process-monitor";

//#[derive(Default)]
pub struct Config {
    columns_config: Option<String>
}

impl Default for Config {
    fn default() -> Self {
        let config = match build_proj_dir() {
            Ok(_) => {
                match read_config() {
                    Ok(s)  => { 
                        debug!("Creating `Config` with {s}");
                        Config { columns_config: Some(s) }
                    }
                    Err(e) => {
                        warn!("Error when attempting to read_config(): {e}");
                        debug!("Creating `Config` with default parameters");
                        Config { columns_config: None }
                    }
                }
            }
            Err(e) => {
                warn!("Error when attempting to build_proj_dir(): {e}");
                debug!("Creating `Config` with default parameters");
                Config { columns_config: None }
            }
        };
        config
    }
}

impl Config {
    pub fn get_columns_config(&self) -> Option<&str> {
        self.columns_config.as_deref()
    }
}

pub fn build_proj_dir() -> Result<()> {
    debug!("`Config:` build_proj_dir(): beginning of function");
    if let Some(proj_dir) = ProjectDirs::from("", "", SOFTWARE_NAME) {
        let config_dir = proj_dir.config_dir();
        debug!("`Config:` build_proj_dir(): config_dir = {config_dir:?}");
        match fs::exists(config_dir) {
            Ok(true) => {
                debug!("`Config:` build_proj_dir(): config_dir already exists");
                return Ok(())
            }
            Ok(false) => {
                debug!("`Config:` build_proj_dir(): config_dir does not yet exist");
                match fs::create_dir_all(config_dir) {
                    Ok(_) => {
                        debug!("`Config:` build_proj_dir(): successfully built project config directory: {config_dir:?}");
                        return Ok(())
                    }
                    Err(e) => {
                        warn!("`Config:` build_proj_dir(): failed to build: {config_dir:?}");
                        return Err(e.into())
                    }
                }
            }
            Err(e) => {
                warn!("`Config:` build_proj_dir(): config_dir: {config_dir:?} existence could not be verified");
                return Err(e.into())
            }
        }

    } else {
        warn!("`Config:` build_proj_dir(): valid $HOME path could not be found");
        let e: std::io::Error = std::io::Error::new(ErrorKind::NotFound, "Valid $HOME path could not be found");
        return Err(e.into())
    }
}

pub fn read_config() -> Result<String> {
    debug!("`Config:` read_config(): beginning of function");
    if let Some(proj_dir) = ProjectDirs::from("", "", SOFTWARE_NAME) {
        let config_dir = proj_dir.config_dir();
        let abs_path = config_dir.join(FILE_NAME);
        debug!("`Config:` read_config(): abs_path = {abs_path:?}");

       if abs_path.is_file() && abs_path.file_name().unwrap_or_default() == FILE_NAME {
            let content = fs::read_to_string(&abs_path)?;
            debug!("`Config:` read_config(): read {content} from {abs_path:?}");
            return Ok(content)
        } else {
            warn!("`Config:` read_config(): {abs_path:?} is not a regular file");
        }
    } else {
        warn!("`Config:` read_config(): valid $HOME path could not be found");
    }

    Ok(String::new())
} 

pub fn write_config(content: String) -> Result<()> {
    debug!("`Config:` write_config(): beginning of function");
    if let Some(proj_dir) = ProjectDirs::from("", "", "process-monitor") {
        let config_dir = proj_dir.config_dir();
        let abs_path = config_dir.join(FILE_NAME);
        debug!("`Config:` write_config(): abs_path = {abs_path:?}");
        fs::write(abs_path, &content)?;
    }

    Ok(())
}

// TODO Write tests and cleanup