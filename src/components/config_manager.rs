use crate::config::AppConfig;

//TODO:
// 1. Save current process table config
// 2. Change refresh rate

pub struct ConfigManagerComponent {
    config: AppConfig
}

impl ConfigManagerComponent {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config
        }
    }
}