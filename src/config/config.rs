use anyhow::{Result};
use log::debug;
use std::fs;
use directories::ProjectDirs;

//#[derive(Default)]
pub struct Config {
    contents: String
}

impl Default for Config {
    fn default() -> Self {
        let config = read_config();
        match config {
            Ok(s)  => { 
                debug!("Creating `Config` with {s}");
                Config { contents: s }
            }
            Err(_) => {
                debug!("Creating `Config` with default parameters");
                Config { contents: String::new() }
            }
        }
    }
}

impl Config {
    pub fn get_contents(&self) -> &str {
        &self.contents
    }
}

const FILE_NAME: &str = "process_monitor.toml";

// Function to read TOML format into a String
pub fn read_config() -> Result<String> {
    debug!("`Config:` read_config(): beginning of function");
    if let Some(proj_dir) = ProjectDirs::from("", "", "process-monitor") {
        let config_dir = proj_dir.config_dir();
        let abs_path = config_dir.join(FILE_NAME);
        debug!("`Config:` read_config(): abs_path = {abs_path:?}");

        if abs_path.is_file() && abs_path.file_name().unwrap_or_default() == FILE_NAME {
            let content = fs::read_to_string(&abs_path)?;
            debug!("`Config:` read_config(): read {content} from {abs_path:?}");
            return Ok(content)
        }
    }

    Ok(String::new())
} 

// Function to write content to config file
pub fn write_config(content: String) -> Result<()> {
    debug!("`Config:` write_config(): beginning of function");
    if let Some(proj_dir) = ProjectDirs::from("", "", "process-monitor") {
        let config_dir = proj_dir.config_dir();
        let abs_path = config_dir.join(FILE_NAME);
        debug!("`Config:` write_config(): abs_path = {abs_path:?}");

        if abs_path.is_file() && abs_path.file_name().unwrap_or_default() == FILE_NAME {
            debug!("`Config:` write_config(): writing {content} to {abs_path:?}");
            fs::write(abs_path, &content)?;
        }
    }

    Ok(())
}

// TODO [5/11/26] Write tests