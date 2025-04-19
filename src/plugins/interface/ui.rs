use crate::events::star_delete::StarDeleteEvent;
use crate::events::star_unselect::StarUnselectEvent;
use crate::resources::selected_star::SelectedStar;
use crate::resources::settings::bloom::BloomSettings;
use crate::resources::settings::graphics::GraphicsSettings;
use crate::resources::settings::wireframe::WireframeSettings;
use crate::resources::window_manager::WindowManager;
use crate::ui::elements::top_bar::draw_top_bar;
use crate::ui::modals::generate_cubic::draw_generate_cubic_modal;
use crate::ui::windows::bloom_settings::draw_bloom_settings_window;
use crate::ui::windows::diagnostics::draw_diagnostics_window;
use crate::ui::windows::graphics_settings::draw_graphics_settings_window;
use crate::ui::windows::selected_star::draw_selected_star_window;
use crate::ui::windows::settings::draw_settings_window;
use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::{EventWriter, Res, ResMut};
use bevy_egui::EguiContexts;

#[allow(clippy::too_many_arguments)]
pub fn user_interface(
    mut contexts: EguiContexts,
    mut window_manager: ResMut<WindowManager>,
    diagnostics: Res<DiagnosticsStore>,
    selected_star: Res<SelectedStar>,
    mut bloom_settings: ResMut<BloomSettings>,
    mut graphics_settings: ResMut<GraphicsSettings>,
    mut wireframe_settings: ResMut<WireframeSettings>,
    mut star_delete_event: EventWriter<StarDeleteEvent>,
    mut star_unselect_event: EventWriter<StarUnselectEvent>,
) {
    let Some(ctx) = contexts.try_ctx_mut() else {
        return;
    };

    draw_top_bar(ctx, &mut window_manager);

    // Windows
    if window_manager.bloom_settings {
        draw_bloom_settings_window(ctx, &mut window_manager, &mut bloom_settings);
    }

    if window_manager.diagnostics {
        draw_diagnostics_window(ctx, &mut window_manager, &diagnostics);
    }

    if window_manager.graphics_settings {
        draw_graphics_settings_window(ctx, &mut window_manager, &mut graphics_settings);
    }

    if selected_star.get_id().is_some() {
        draw_selected_star_window(
            ctx,
            &selected_star,
            &mut star_delete_event,
            &mut star_unselect_event,
        );
    }

    if window_manager.settings {
        draw_settings_window(
            ctx,
            &mut window_manager,
            &mut bloom_settings,
            &mut wireframe_settings,
        );
    }

    // Modals
    if window_manager.generate_cubic_modal {
        draw_generate_cubic_modal(ctx, &mut window_manager);
    }
}
