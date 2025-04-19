use crate::plugins::startup::setup::setup;
use bevy::app::App;
use bevy::prelude::{Plugin, Startup};

mod setup;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
