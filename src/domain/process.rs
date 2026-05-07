pub mod primitive;
pub mod model;

use crate::domain::process::model::ProcessSnapShot;

pub trait ProcessSnapShotSource {
    fn fetch_process_snapshot(&self) -> ProcessSnapShot;
}
