use crate::physics::objects::star::Star;
use bevy::math::Vec3;
use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct SelectedStar {
    id: Option<u64>,
    position: Option<Vec3>,
    data: Option<Star>,
}

impl SelectedStar {
    pub fn clear(&mut self) {
        self.id = None;
        self.position = None;
        self.data = None;
    }

    pub fn get_id(&self) -> Option<u64> {
        self.id
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }

    pub fn get_position(&self) -> Option<Vec3> {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = Some(position);
    }

    pub fn get_data(&self) -> &Option<Star> {
        &self.data
    }

    pub fn set_data(&mut self, data: Star) {
        self.data = Some(data);
    }
}
