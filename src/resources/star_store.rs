use crate::physics::objects::star::Star;
use crate::types::cluster_generation_method::ClusterGenerationMethod;
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
    cluster_method: ClusterGenerationMethod,
    cluster_method_string: String,
}

impl StarStore {
    pub fn set_cluster_method(&mut self, method: &ClusterGenerationMethod) {
        self.rng = StdRng::seed_from_u64(method.get_seed());
        self.cluster_method = method.clone();
        self.cluster_method_string = self.cluster_method.encode();
    }

    pub fn get_rng(&mut self) -> &mut StdRng {
        &mut self.rng
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

    pub fn get_star_count(&self) -> usize {
        self.stars.len()
    }

    pub fn get_cluster_method_string(&self) -> &str {
        &self.cluster_method_string
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
            cluster_method: Default::default(),
            cluster_method_string: String::new(),
        }
    }
}
