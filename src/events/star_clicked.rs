use bevy::prelude::Event;

#[derive(Debug, Event)]
pub struct StarClickedEvent(u64);

impl StarClickedEvent {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn get_id(&self) -> u64 {
        self.0
    }
}
