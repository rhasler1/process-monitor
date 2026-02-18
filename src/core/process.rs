pub mod primitive;
pub mod model;

use crate::core::process::model::ProcessSnapShot;
// TODO 2/18/2026 - Semantics are slightly off here;
// ProcessSnapShot is a snap shot of all system processes
// For now this is ok, but in the future it might be
// desirable to get a snapshot of one process via providing
// a PID. Change semantics then.
pub trait ProcessSnapShotSource {
    fn fetch_process_snapshot(&self) -> ProcessSnapShot;
}
