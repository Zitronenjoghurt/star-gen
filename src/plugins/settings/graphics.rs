use crate::resources::settings::graphics::GraphicsSettings;
use bevy::prelude::{Camera3d, Projection, Query, Res, Single, Window, With};
use bevy::render::view::VisibilityRange;
use bevy::window::PresentMode;

pub fn apply_graphics_settings(
    mut window: Single<&mut Window>,
    settings: Res<GraphicsSettings>,
    mut cameras: Query<&mut Projection, With<Camera3d>>,
    mut visibility_ranges: Query<&mut VisibilityRange>,
) {
    if settings.vsync {
        window.present_mode = PresentMode::AutoVsync;
    } else {
        window.present_mode = PresentMode::AutoNoVsync;
    }

    for mut projection in cameras.iter_mut() {
        if let Projection::Perspective(perspective) = projection.as_mut() {
            perspective.far = settings.render_distance;
        }
    }

    for mut visibility_range in visibility_ranges.iter_mut() {
        visibility_range.end_margin = (settings.render_distance - 1.0)..(settings.render_distance);
    }
}
