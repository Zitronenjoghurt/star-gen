use crate::resources::settings::bloom::BloomSettings;
use crate::resources::settings::wireframe::WireframeSettings;
use bevy::prelude::{ButtonInput, KeyCode, Res, ResMut};

pub const WIREFRAME_KEYCODE: KeyCode = KeyCode::KeyW;
pub const BLOOM_KEYCODE: KeyCode = KeyCode::KeyB;

pub fn receive_keyboard_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut wireframe_settings: ResMut<WireframeSettings>,
    mut bloom_settings: ResMut<BloomSettings>,
) {
    if keys.just_pressed(WIREFRAME_KEYCODE) {
        wireframe_settings.toggle();
    }
    if keys.just_pressed(BLOOM_KEYCODE) {
        bloom_settings.toggle();
    }
}
