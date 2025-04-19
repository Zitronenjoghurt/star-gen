use crate::physics::objects::star::Star;
use bevy::prelude::{Entity, Resource};
use rand::prelude::StdRng;
use rand::SeedableRng;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Resource)]
pub struct StarStore {
    new_id: u64,
    stars: HashMap<u64, Star>,
    entities: HashMap<u64, Entity>,
    rng: StdRng,
    seed: u64,
}

impl StarStore {
    pub fn set_rng_seed(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
        self.seed = seed;
    }

    pub fn get_rng(&mut self) -> &mut StdRng {
        &mut self.rng
    }

    pub fn get_seed(&mut self) -> u64 {
        self.seed
    }

    pub fn add_star(&mut self, star: Star) -> u64 {
        let id = self.new_id;
        self.stars.insert(id, star);
        self.new_id += 1;
        id
    }

    pub fn delete_star(&mut self, id: u64) -> Option<Entity> {
        self.stars.remove(&id);
        self.entities.remove(&id)
    }

    pub fn delete_all_stars(&mut self) -> impl Iterator<Item = Entity> {
        let entities = self.entities.drain().map(|(_, entity)| entity);
        self.stars.clear();
        self.new_id = 0;
        entities
    }

    pub fn add_entity(&mut self, id: u64, entity: Entity) {
        self.entities.insert(id, entity);
    }

    pub fn get_entity(&self, id: u64) -> Option<Entity> {
        self.entities.get(&id).copied()
    }
}

impl Default for StarStore {
    fn default() -> Self {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let seed = duration.as_secs();
        Self {
            new_id: 0,
            stars: HashMap::new(),
            entities: HashMap::new(),
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }
}
