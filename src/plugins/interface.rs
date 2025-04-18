use crate::plugins::interface::absorb_egui_inputs::absorb_egui_inputs;
use crate::plugins::interface::ui::user_interface;
use bevy::prelude::{App, IntoSystemConfigs, Plugin, PreUpdate, Update};
use bevy_egui::EguiPreUpdateSet;

mod absorb_egui_inputs;
mod ui;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            absorb_egui_inputs
                .after(bevy_egui::input::write_egui_input_system)
                .before(bevy_egui::begin_pass_system),
        )
        .add_systems(Update, user_interface.after(EguiPreUpdateSet::InitContexts));
    }
}
