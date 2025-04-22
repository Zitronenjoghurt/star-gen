use crate::events::star_cluster_generate::StarClusterGenerateEvent;
use crate::persistence::{load_state, save_state_exists};
use crate::types::cluster_generation_method::ClusterGenerationMethod;
use bevy::log::info;
use bevy::prelude::{Events, World};

pub fn load_on_startup(world: &mut World) {
    if save_state_exists() {
        info!("Save state found... trying to load state");
        load_state(world);
    } else {
        info!("No save state found... continuing with default");
    }
}
