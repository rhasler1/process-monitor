pub mod app_event;

#[derive(PartialEq, Debug)]
pub enum EventState {
    Consumed,
    ReturnPID(u32),
    ReturnColumns(String),
    NotConsumed
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        *self == Self::Consumed
    }

    pub fn is_return_pid(&self) -> bool {
        match self {
            EventState::Consumed => false,
            EventState::ReturnPID(_) => true,
            EventState::NotConsumed => false,
            EventState::ReturnColumns(_) => false
        }
    }

    pub fn pid(&self) -> Option<u32> {
        match self {
            EventState::Consumed => None,
            EventState::ReturnPID(pid) => Some(*pid),
            EventState::NotConsumed => None,
            EventState::ReturnColumns(_cols) => None,
        }
    }

    pub fn is_return_cols(&self) -> bool {
        match self {
            EventState::Consumed => false,
            EventState::ReturnPID(_) => false,
            EventState::NotConsumed => false,
            EventState::ReturnColumns(_) => true
        }
    }

    pub fn cols(&self) -> Option<String> {
        match self {
            EventState::Consumed => None,
            EventState::ReturnPID(_pid) => None,
            EventState::NotConsumed => None,
            EventState::ReturnColumns(cols) => Some(cols.clone()),
        }
    }
}
