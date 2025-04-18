use crate::resources::simulation_settings::SimulationSettings;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::{ButtonInput, Camera, KeyCode, Query, Res, ResMut};

pub const WIREFRAME_KEYCODE: KeyCode = KeyCode::KeyW;
pub const BLOOM_KEYCODE: KeyCode = KeyCode::KeyB;

pub fn receive_keyboard_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut simulation_settings: ResMut<SimulationSettings>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut camera: Query<&mut Camera>,
) {
    if keys.just_pressed(WIREFRAME_KEYCODE) {
        toggle_wireframe_mode(&mut simulation_settings, &mut wireframe_config);
    }
    if keys.just_pressed(BLOOM_KEYCODE) {
        toggle_bloom(&mut simulation_settings, &mut camera)
    }
}

fn toggle_wireframe_mode(
    simulation_settings: &mut ResMut<SimulationSettings>,
    wireframe_config: &mut ResMut<WireframeConfig>,
) {
    simulation_settings.wireframe = !simulation_settings.wireframe;
    wireframe_config.global = simulation_settings.wireframe;
}

fn toggle_bloom(
    simulation_settings: &mut ResMut<SimulationSettings>,
    camera: &mut Query<&mut Camera>,
) {
    simulation_settings.bloom = !simulation_settings.bloom;

    for mut camera in camera.iter_mut() {
        camera.hdr = simulation_settings.bloom;
    }
}
