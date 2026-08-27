use std::fmt::Display;

use crate::config::AppConfig;
use crate::adapters::crossterm::input::Key;
use anyhow::{Result, anyhow};

use super::Event;

// TODO:
// 1.   Update Refresh Rate: Config Manager
//      should return new refresh rate to
//      main to modify app_events interval
//      Additionally, ProcessTableInterval
//      needs to be updated.
//
// 2.   Save: Config Manager should return new 
//      RefreshRateConfig & ViewsConfig to
//      main for config_worker to write.

pub enum ConfigMenuOptions {
    SaveCurrentConfig,
    SetRefreshRate1s
}

impl Display for ConfigMenuOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SaveCurrentConfig => write!(f, "Save current config"),
            Self::SetRefreshRate1s => write!(f, "Set refresh rate to 1s")
        }
    }
}

pub struct ConfigManagerMenu {
    menu_options:   Vec<ConfigMenuOptions>,
    menu_selection: usize
}

impl Default for ConfigManagerMenu {
    fn default() -> Self {
        Self {
            menu_options: vec![
                ConfigMenuOptions::SaveCurrentConfig,
                ConfigMenuOptions::SetRefreshRate1s
            ],
            menu_selection: 0
        }
    }
}

impl ConfigManagerMenu {
    // Menu options length is fixed to 2
    fn inc_selection(&mut self) {
        if self.menu_selection < self.menu_options.len() - 1 {
            self.menu_selection += 1;
        }
    }

    fn dec_selection(&mut self) {
        self.menu_selection = 
            self.menu_selection.saturating_sub(1);
    }

    pub fn menu_options(&self) -> &Vec<ConfigMenuOptions> {
        &self.menu_options
    }

    pub fn menu_selection(&self) -> usize {
        self.menu_selection
    }
}

pub struct ConfigManagerComponent {
    config: AppConfig,
    menu:   ConfigManagerMenu
}

impl ConfigManagerComponent {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            menu: ConfigManagerMenu::default()
        }
    }

    /// Serialize current config and return String
    pub fn serialize_current_config(&self) -> Result<String> {
        toml::to_string(&self.config).map_err(|e| {
            anyhow!("{e}")
        })
    }

    pub fn mut_app_config(&mut self) -> &mut AppConfig {
        &mut self.config
    }

    pub fn mut_menu(&mut self) -> &mut ConfigManagerMenu {
        &mut self.menu
    }

    pub fn menu(&self) -> &ConfigManagerMenu {
        &self.menu
    }
}

pub enum ConfigManagerEventState {
    Consumed,
    NotConsumed,
    SaveCurrentConfig,
}

impl Event for ConfigManagerComponent {
    type EventState = ConfigManagerEventState;

    fn event(&mut self, key: Key) -> Result<Self::EventState> {
        match key {
            Key::Up => {
                self.mut_menu().dec_selection();  
            }
            Key::Down => {
                self.mut_menu().inc_selection();
            }
            Key::Enter => {
                let event = self.menu()
                    .menu_options
                    .get(self.menu().menu_selection)
                    .unwrap();

                match event {
                    ConfigMenuOptions::SaveCurrentConfig => {
                        return Ok(ConfigManagerEventState::SaveCurrentConfig);
                    }
                    ConfigMenuOptions::SetRefreshRate1s => {
                        //TODO
                    }
                }
            }

            _ => { return Ok(Self::EventState::NotConsumed); }
        }

        Ok(Self::EventState::Consumed)
    }
}


