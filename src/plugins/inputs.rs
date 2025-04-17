use crate::plugins::inputs::keyboard_inputs::receive_keyboard_inputs;
use bevy::app::App;
use bevy::prelude::{Plugin, Update};

mod keyboard_inputs;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, receive_keyboard_inputs);
    }
}
