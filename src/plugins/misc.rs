use crate::plugins::misc::observe_pan_orbit::observe_pan_orbit_camera;
use bevy::app::App;
use bevy::prelude::{Plugin, Update};

pub mod observe_pan_orbit;

pub struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, observe_pan_orbit_camera);
    }
}
