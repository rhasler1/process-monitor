pub mod app_event;

#[derive(PartialEq)]
pub enum EventState {
    Consumed,
    ReturnPayload(u32),
    NotConsumed
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        *self == Self::Consumed
    }

    pub fn is_return_payload(&self) -> bool {
        match self {
            EventState::Consumed => false,
            EventState::ReturnPayload(_) => true,
            EventState::NotConsumed => false
        }
    }

    pub fn payload(&self) -> Option<u32> {
        match self {
            EventState::Consumed => None,
            EventState::ReturnPayload(pid) => Some(*pid),
            EventState::NotConsumed => None
        }
    }
}
