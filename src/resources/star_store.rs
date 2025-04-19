use crate::physics::objects::star::Star;
use bevy::prelude::{Entity, Resource};
use std::collections::HashMap;

#[derive(Debug, Default, Resource)]
pub struct StarStore {
    last_id: u64,
    stars: HashMap<u64, Star>,
    entities: HashMap<u64, Entity>,
}

impl StarStore {
    pub fn add_star(&mut self, star: Star) -> u64 {
        let id = self.last_id;
        self.stars.insert(id, star);
        self.last_id += 1;
        id
    }

    pub fn delete_star(&mut self, id: u64) -> Option<Entity> {
        self.stars.remove(&id);
        self.entities.remove(&id)
    }

    pub fn add_entity(&mut self, id: u64, entity: Entity) {
        self.entities.insert(id, entity);
    }

    pub fn get_entity(&self, id: u64) -> Option<Entity> {
        self.entities.get(&id).copied()
    }
}
