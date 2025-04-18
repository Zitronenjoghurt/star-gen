use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct SelectedPoint {
    id: Option<u64>,
}

impl SelectedPoint {
    pub fn get_id(&self) -> Option<u64> {
        self.id
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }
}
