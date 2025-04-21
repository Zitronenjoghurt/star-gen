use crate::utils::persistence::save_state;
use bevy::prelude::{info, AppExit, EventReader, World};

pub fn save_on_exit(mut exit_event: EventReader<AppExit>, world: &World) {
    let Some(_) = exit_event.read().last() else {
        return;
    };

    info!("Game is closing, saving state...");
    save_state(world);
}
