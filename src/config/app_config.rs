// [5/14] `Config` only concern is the column configuration
// At some point `Config` should be expanded to include key
// configuration, refresh rate, and snapshot rebuild rate.


use anyhow::{anyhow, Result};
use log::{debug, warn};
use std::fs;
use directories::ProjectDirs;

const AUTHOR:        &str = "rhasler1";
const DOMAIN:        &str = "io";
const SOFTWARE_NAME: &str = "process-monitor";
const FILE_NAME:     &str = "process_monitor.toml";

pub struct Config {
    columns_config: Option<String>
}

impl Default for Config {
    fn default() -> Self {
        
        match build_proj_dir() {
            Ok(_) => {
                match read_config() {
                    Ok(s)  => { 
                        debug!("`Config:` default(): Creating with columns_config parameter:\n{s:?}");
                        Config { columns_config: s }
                    }
                    Err(e) => {
                        warn!("Error when attempting to read_config(): {e}");
                        debug!("`Config:` default(): Creating with default columns_config parameter: None");
                        Config { columns_config: None }
                    }
                }
            }
            Err(e) => {
                warn!("`Config:` default(): Error when attempting to build_proj_dir(): {e}");
                debug!("`Config:` default(): Creating `Config` with default parameters");
                Config { columns_config: None }
            }
        }
    }
}

impl Config {
    pub fn get_columns_config(&self) -> Option<&str> {
        self.columns_config.as_deref()
    }
}

// Build project configuration directory if it does not
// already exist. `ProjectDirs::from()` provides the
// OS standard path for the project directory.
// Return value of Ok signals the configuration directory
// exists.
pub fn build_proj_dir() -> Result<()> {
    debug!("`Config:` build_proj_dir(): beginning of function");
    if let Some(proj_dir) = ProjectDirs::from(DOMAIN, AUTHOR, SOFTWARE_NAME) {
        let config_dir = proj_dir.config_dir();
        debug!("`Config:` build_proj_dir(): config_dir = {config_dir:?}");
        match fs::exists(config_dir) {
            Ok(true) => {
                debug!("`Config:` build_proj_dir(): config_dir already exists");
                Ok(())
            }
            Ok(false) => {
                debug!("`Config:` build_proj_dir(): config_dir does not yet exist");
                match fs::create_dir_all(config_dir) {
                    Ok(_) => {
                        debug!("`Config:` build_proj_dir(): successfully built project config directory: {config_dir:?}");
                        Ok(())
                    }
                    Err(e) => {
                        warn!("`Config:` build_proj_dir(): failed to build: {config_dir:?}");
                        Err(e.into())
                    }
                }
            }
            Err(e) => {
                warn!("`Config:` build_proj_dir(): config_dir: {config_dir:?} existence could not be verified");
                Err(e.into())
            }
        }

    } else {
        warn!("`Config:` build_proj_dir(): valid $HOME path could not be found");
        Err(anyhow!("$HOME directory not found"))
    }
}

// Read the entire contents of the configuration file
// Returns Some(String)
pub fn read_config() -> Result<Option<String>> {
    debug!("`Config:` read_config(): beginning of function");
    if let Some(proj_dir) = ProjectDirs::from(DOMAIN, AUTHOR, SOFTWARE_NAME) {
        let config_dir = proj_dir.config_dir();
        let abs_path = config_dir.join(FILE_NAME);
        debug!("`Config:` read_config(): abs_path = {abs_path:?}");
        // Second conditional is redundant
        if abs_path.is_file() && abs_path.file_name().unwrap_or_default() == FILE_NAME {
            let content = fs::read_to_string(&abs_path)?;
            debug!("`Config:` read_config(): read {content} from {abs_path:?}");
            return Ok(Some(content))
        } else {
            warn!("`Config:` read_config(): {abs_path:?} is not a regular file");
        }
    } else {
        warn!("`Config:` read_config(): valid $HOME path could not be found");
    }

    Ok(None)
} 


// Replace the entire contents of the configuration file
pub fn write_config(content: String) -> Result<()> {
    debug!("`Config:` write_config(): beginning of function");
    if let Some(proj_dir) = ProjectDirs::from(DOMAIN, AUTHOR, SOFTWARE_NAME) {
        let config_dir = proj_dir.config_dir();
        let abs_path = config_dir.join(FILE_NAME);
        debug!("`Config:` write_config(): abs_path = {abs_path:?}");
        // fs::write() creates the file if it does not already exist
        fs::write(abs_path, &content)?;
    }

    Ok(())
}
