use crate::components::process_table::ProcessTableViews;

use std::time::Duration;

use anyhow::{anyhow, Result};

#[derive(Debug, Default)]
pub struct ProcessTablesViewsSerialized(String);

impl ProcessTablesViewsSerialized {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub struct ProcessRefreshRate(Duration);

impl ProcessRefreshRate {
    pub fn new(d: Duration) -> Result<Self> {
        if Self::valid_refresh_rates()
            .contains(&d) {
                Ok(Self(d))
        } else {
            Err(anyhow!("Invalid process refresh rate duration: {:?}", d))
        }
    }

    const fn valid_refresh_rates() -> [Duration; 3] {
        [
            Duration::from_secs(1),
            Duration::from_secs(2),
            Duration::from_secs(5)
        ]
    }
}



pub struct Config {
    views_serialized: Option<ProcessTablesViewsSerialized>,
}

/*
 *
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessPid(u32);

impl ProcessPid {
    pub fn new(pid: u32) -> Self {
        Self(pid)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn as_u64(&self) -> u64 {
        self.0 as u64
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}
 *
 * */
