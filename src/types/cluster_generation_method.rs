use crate::events::star_spawn::StarSpawnEvent;
use crate::resources::star_store::StarStore;
use crate::types::cluster_generation_settings::cubic::CubicClusterGenerationSettings;
use bevy::prelude::{EventWriter, ResMut};

#[derive(Debug, Default, Clone)]
pub enum ClusterGenerationMethod {
    #[default]
    None,
    Cubic(CubicClusterGenerationSettings),
}

impl ClusterGenerationMethod {
    pub fn get_seed(&self) -> u64 {
        match self {
            ClusterGenerationMethod::None => 0,
            ClusterGenerationMethod::Cubic(cubic) => cubic.seed,
        }
    }

    pub fn encode(&self) -> String {
        match self {
            ClusterGenerationMethod::None => String::new(),
            ClusterGenerationMethod::Cubic(cubic) => cubic.encode(),
        }
    }

    pub fn decode(encoded: &str) -> Option<ClusterGenerationMethod> {
        if encoded.starts_with("cubic") {
            let cubic = CubicClusterGenerationSettings::decode(encoded)?;
            return Some(ClusterGenerationMethod::Cubic(cubic));
        }
        None
    }

    pub fn cubic_with_seed(seed: u64) -> Self {
        Self::Cubic(CubicClusterGenerationSettings::default_with_seed(seed))
    }

    pub fn generate(
        &self,
        star_store: &mut ResMut<StarStore>,
        star_spawn_event: &mut EventWriter<StarSpawnEvent>,
    ) {
        star_store.regenerate(self);
        match self {
            Self::None => {}
            Self::Cubic(cubic) => cubic.generate(star_store, star_spawn_event),
        }
    }
}
