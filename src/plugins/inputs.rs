use crate::plugins::inputs::keyboard_inputs::receive_keyboard_inputs;
use crate::plugins::inputs::point_selection::point_selection;
use bevy::app::App;
use bevy::prelude::{Plugin, Update};

mod keyboard_inputs;
mod point_selection;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (receive_keyboard_inputs, point_selection));
    }
}
