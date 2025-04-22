use crate::persistence::star_state::StarState;
use crate::resources::star_store::StarStore;
use crate::types::cluster_generation_method::ClusterGenerationMethod;
use bevy::prelude::World;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StarClusterState {
    id: String,
    name: String,
    method_string: String,
    stars: Vec<StarState>,
}

impl StarClusterState {
    pub fn save(world: &World) -> Self {
        let star_store = world.resource::<StarStore>();
        let stars = star_store
            .get_stars()
            .iter()
            .map(|(id, star)| StarState::save(*id, star))
            .collect();
        Self {
            id: star_store.get_cluster_id().to_string(),
            name: star_store.get_cluster_name().to_string(),
            method_string: star_store.get_cluster_method_string().to_string(),
            stars,
        }
    }

    pub fn load(&self, world: &mut World) {
        if let Some(mut star_store) = world.get_resource_mut::<StarStore>() {
            let method = ClusterGenerationMethod::decode(&self.method_string).unwrap_or_default();
            star_store.regenerate(&method);

            star_store.set_cluster_id(&self.id);
            star_store.set_cluster_name(&self.name);
            self.stars
                .iter()
                .for_each(|star_state| star_state.load(world));
        }
    }
}
