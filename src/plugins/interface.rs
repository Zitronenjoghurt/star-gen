use crate::plugins::interface::absorb_egui_inputs::absorb_egui_inputs;
use crate::resources::selected_star::SelectedStar;
use crate::resources::ui::generate_cubic_modal_state::GenerateCubicModalState;
use crate::resources::ui::generate_seed_modal_state::GenerateSeedModalState;
use crate::resources::window_manager::WindowManager;
use crate::ui::misc::top_bar::render_top_bar;
use crate::ui::modals::generate_cubic::render_generate_cubic_modal;
use crate::ui::modals::generate_seeded::render_generate_seeded_modal;
use crate::ui::windows::bloom_settings::render_bloom_settings_window;
use crate::ui::windows::cluster_info::render_cluster_info_window;
use crate::ui::windows::control_settings::render_control_settings_window;
use crate::ui::windows::diagnostics::render_diagnostics_window;
use crate::ui::windows::graphics_settings::render_graphics_settings_window;
use crate::ui::windows::selected_star::render_selected_star_window;
use crate::ui::windows::settings::render_settings_window;
use bevy::prelude::{App, IntoSystemConfigs, Plugin, PreUpdate, Res, Update};

mod absorb_egui_inputs;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            absorb_egui_inputs
                .after(bevy_egui::input::write_egui_input_system)
                .before(bevy_egui::begin_pass_system),
        )
        .add_systems(Update, render_top_bar)
        .add_systems(
            Update,
            (
                render_bloom_settings_window
                    .run_if(|window_manager: Res<WindowManager>| window_manager.bloom_settings),
                render_cluster_info_window
                    .run_if(|window_manager: Res<WindowManager>| window_manager.cluster_info),
                render_control_settings_window
                    .run_if(|window_manager: Res<WindowManager>| window_manager.control_settings),
                render_diagnostics_window
                    .run_if(|window_manager: Res<WindowManager>| window_manager.diagnostics),
                render_graphics_settings_window
                    .run_if(|window_manager: Res<WindowManager>| window_manager.graphics_settings),
                render_settings_window
                    .run_if(|window_manager: Res<WindowManager>| window_manager.settings),
                render_selected_star_window
                    .run_if(|selected_star: Res<SelectedStar>| selected_star.get_id().is_some()),
            ),
        )
        .insert_resource(GenerateCubicModalState::default())
        .insert_resource(GenerateSeedModalState::default())
        .add_systems(
            Update,
            (
                render_generate_cubic_modal.run_if(|window_manager: Res<WindowManager>| {
                    window_manager.generate_cubic_modal
                }),
                render_generate_seeded_modal.run_if(|window_manager: Res<WindowManager>| {
                    window_manager.generate_seeded_modal
                }),
            ),
        );
    }
}
