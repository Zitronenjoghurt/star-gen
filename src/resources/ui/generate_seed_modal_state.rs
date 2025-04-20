use crate::types::cluster_generation_method::ClusterGenerationMethod;
use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct GenerateSeedModalState {
    pub seed: String,
    pub method: Option<ClusterGenerationMethod>,
}

impl GenerateSeedModalState {
    pub fn validate(&mut self) {
        self.method = ClusterGenerationMethod::decode(&self.seed);
    }
}
