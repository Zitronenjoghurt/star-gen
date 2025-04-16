use bevy::prelude::Component;

#[derive(Debug, Default, Component)]
pub struct SelectablePoint {
    id: u64,
    selected: bool,
}

impl SelectablePoint {
    pub fn new(id: u64, selected: bool) -> Self {
        Self { id, selected }
    }
}
