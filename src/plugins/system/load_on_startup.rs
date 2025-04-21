use crate::utils::persistence::{load_state, save_state_exists};
use bevy::log::info;
use bevy::prelude::World;

pub fn load_on_startup(world: &mut World) {
    if save_state_exists() {
        info!("Save state found... trying to load state");
        load_state(world);
    } else {
        info!("No save state found... continuing with default");
    }
}
