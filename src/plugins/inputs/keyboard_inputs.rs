use crate::resources::simulation_settings::SimulationSettings;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::{ButtonInput, KeyCode, Res, ResMut};

pub const WIREFRAME_KEYCODE: KeyCode = KeyCode::KeyW;

pub fn receive_keyboard_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    simulation_settings: ResMut<SimulationSettings>,
    wireframe_config: ResMut<WireframeConfig>,
) {
    if keys.just_pressed(WIREFRAME_KEYCODE) {
        toggle_wireframe_mode(simulation_settings, wireframe_config);
    }
}

fn toggle_wireframe_mode(
    mut simulation_settings: ResMut<SimulationSettings>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    simulation_settings.wireframe = !simulation_settings.wireframe;
    wireframe_config.global = simulation_settings.wireframe;
}
