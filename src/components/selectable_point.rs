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

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn unselect(&mut self) {
        self.selected = false;
    }
}
