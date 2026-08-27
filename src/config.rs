pub mod refresh_rate_config;
use std::cell::Ref;

pub use refresh_rate_config::RefreshRateConfig;
pub mod theme_config;

use crate::{components::process_table::ProcessTableViewsConfig};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    refresh_rate_config: RefreshRateConfig,
    table_views_config:  Option<ProcessTableViewsConfig>,
}

impl AppConfig {
    pub fn new(
        refresh_rate_config:    RefreshRateConfig,
        table_views_config:     Option<ProcessTableViewsConfig>
    ) -> Self {
        Self {
            refresh_rate_config,
            table_views_config
        }
    }
    pub fn update_table_views_config(&mut self, config: &ProcessTableViewsConfig) {
        self.table_views_config = Some(config.clone());
    }

    pub fn refresh_rate_config(&self) -> &RefreshRateConfig {
        &self.refresh_rate_config
    }

    pub fn tables_views_config(&self) -> &Option<ProcessTableViewsConfig> {
        &self.table_views_config
    }
}

