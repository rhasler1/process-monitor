pub mod app_event;

#[derive(PartialEq)]
pub enum EventState {
    Consumed,
    NotConsumed
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        *self == Self::Consumed
    }
}
