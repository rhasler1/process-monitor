pub mod application;
pub mod events;
pub mod models;
pub mod components;

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
