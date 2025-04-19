use crate::resources::settings::graphics::GraphicsSettings;
use bevy::prelude::{Res, Single, Window};
use bevy::window::PresentMode;

pub fn apply_graphics_settings(mut window: Single<&mut Window>, settings: Res<GraphicsSettings>) {
    if settings.vsync {
        window.present_mode = PresentMode::AutoVsync;
    } else {
        window.present_mode = PresentMode::AutoNoVsync;
    }
}
