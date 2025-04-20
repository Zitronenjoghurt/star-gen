use crate::events::star_cluster_generate::StarClusterGenerateEvent;
use crate::types::cluster_generation_method::ClusterGenerationMethod;
use crate::types::cluster_generation_settings::cubic::CubicClusterGenerationSettings;
use crate::utils::base64::{b64_decode_u64, b64_encode_u64};
use crate::utils::random::random_u64;
use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct GenerateCubicModalState {
    pub settings: CubicClusterGenerationSettings,
    pub seed: String,
    pub edit_seed: String,
}

impl GenerateCubicModalState {
    pub fn regenerate_seed(&mut self) {
        self.set_seed(b64_encode_u64(random_u64()));
    }

    pub fn set_seed(&mut self, seed: String) {
        if b64_decode_u64(&seed).is_some() {
            self.seed = seed.clone();
            self.edit_seed = seed;
        } else {
            self.edit_seed = self.seed.clone();
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn get_event(&mut self) -> Option<StarClusterGenerateEvent> {
        let seed = b64_decode_u64(&self.seed)?;
        self.settings.seed = seed;
        let method = ClusterGenerationMethod::Cubic(self.settings.clone());
        Some(StarClusterGenerateEvent::new(method))
    }
}

impl Default for GenerateCubicModalState {
    fn default() -> Self {
        let seed = b64_encode_u64(random_u64());
        Self {
            settings: Default::default(),
            seed: seed.clone(),
            edit_seed: seed,
        }
    }
}
