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
    sorted_mass: Vec<u64>,
    sorted_radius: Vec<u64>,
    sorted_luminosity: Vec<u64>,
    sorted_surface_temperature: Vec<u64>,
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
        self.build_sort_indices();
        id
    }

    pub fn get_star(&self, id: u64) -> Option<&Star> {
        self.stars.get(&id)
    }

    pub fn delete_star(&mut self, id: u64) -> Option<Entity> {
        self.stars.remove(&id);
        self.build_sort_indices();
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

    pub fn build_sort_indices(&mut self) {
        self.sorted_mass.clear();
        self.sorted_radius.clear();
        self.sorted_luminosity.clear();
        self.sorted_surface_temperature.clear();

        self.sorted_mass = self.stars.keys().copied().collect();
        self.sorted_radius = self.stars.keys().copied().collect();
        self.sorted_luminosity = self.stars.keys().copied().collect();
        self.sorted_surface_temperature = self.stars.keys().copied().collect();

        self.sorted_mass.sort_by(|a, b| {
            let mass_a = self.stars.get(a).unwrap().get_mass();
            let mass_b = self.stars.get(b).unwrap().get_mass();
            mass_a
                .partial_cmp(&mass_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.sorted_radius.sort_by(|a, b| {
            let radius_a = self.stars.get(a).unwrap().get_radius();
            let radius_b = self.stars.get(b).unwrap().get_radius();
            radius_a
                .partial_cmp(&radius_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.sorted_luminosity.sort_by(|a, b| {
            let luminosity_a = self.stars.get(a).unwrap().get_luminosity();
            let luminosity_b = self.stars.get(b).unwrap().get_luminosity();
            luminosity_a
                .partial_cmp(&luminosity_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.sorted_surface_temperature.sort_by(|a, b| {
            let temp_a = self.stars.get(a).unwrap().get_surface_temperature();
            let temp_b = self.stars.get(b).unwrap().get_surface_temperature();
            temp_a
                .partial_cmp(&temp_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    pub fn get_id_sorted_by_mass(&self, index: usize, ascending: bool) -> Option<u64> {
        if self.sorted_mass.is_empty() {
            return None;
        }

        let actual_index = if ascending {
            index
        } else {
            self.sorted_mass.len().checked_sub(index)?
        };

        self.sorted_mass.get(actual_index).copied()
    }

    pub fn get_id_sorted_radius(&self, index: usize, ascending: bool) -> Option<u64> {
        if self.sorted_radius.is_empty() {
            return None;
        }

        let actual_index = if ascending {
            index
        } else {
            self.sorted_radius.len().checked_sub(index)?
        };

        self.sorted_radius.get(actual_index).copied()
    }

    pub fn get_id_sorted_luminosity(&self, index: usize, ascending: bool) -> Option<u64> {
        if self.sorted_luminosity.is_empty() {
            return None;
        }

        let actual_index = if ascending {
            index
        } else {
            self.sorted_luminosity.len().checked_sub(index)?
        };

        self.sorted_luminosity.get(actual_index).copied()
    }

    pub fn get_id_sorted_surface_temperature(&self, index: usize, ascending: bool) -> Option<u64> {
        if self.sorted_surface_temperature.is_empty() {
            return None;
        }

        let actual_index = if ascending {
            index
        } else {
            self.sorted_surface_temperature.len().checked_sub(index)?
        };

        self.sorted_surface_temperature.get(actual_index).copied()
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
            sorted_mass: Vec::new(),
            sorted_radius: Vec::new(),
            sorted_luminosity: Vec::new(),
            sorted_surface_temperature: Vec::new(),
        }
    }
}
