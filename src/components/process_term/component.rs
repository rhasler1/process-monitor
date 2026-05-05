use crate::adapters::crossterm::input::Key;
use crate::events::EventState;

#[derive(Default)]
pub struct Component {
    model:      u32,  // pid
}

impl Component {
    pub fn set(&mut self, pid: u32) {
        self.model = pid;
    }
    pub fn key_event(&self, key: Key) -> EventState {
        EventState::NotConsumed
    }

    pub fn is_term_event_ready(&self) -> bool {
        false
    }

    pub fn get_pid_to_terminate(&self) -> u32 {
        0
    }
}
