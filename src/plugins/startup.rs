use crate::plugins::startup::setup::setup;
use crate::plugins::startup::spawn_points::spawn_points;
use bevy::app::{App, PostStartup};
use bevy::prelude::{Plugin, Startup};

mod setup;
mod spawn_points;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostStartup, spawn_points);
    }
}
