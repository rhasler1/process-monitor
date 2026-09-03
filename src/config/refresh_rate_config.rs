use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshRateConfig {
    interval: Duration
}

impl Default for RefreshRateConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(2)
        }
    }
}

const fn valid_intervals() -> [Duration; 3] {
    [
        Duration::from_secs(1),
        Duration::from_secs(2),
        Duration::from_secs(5)
    ]
}

impl RefreshRateConfig {
    pub fn set_interval(
        &mut self,
        interval: Duration
    ) -> bool {
        if valid_intervals().contains(&interval) {
            self.interval = interval;
            true
        } else {
            false
        }
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }
}
