use crate::plugins::settings::bloom::apply_bloom_settings;
use crate::plugins::settings::graphics::apply_graphics_settings;
use crate::plugins::settings::wireframe::apply_wireframe_settings;
use crate::resources::settings::bloom::BloomSettings;
use crate::resources::settings::graphics::GraphicsSettings;
use crate::resources::settings::wireframe::WireframeSettings;
use crate::resources::window_manager::WindowManager;
use bevy::prelude::{resource_changed, App, IntoSystemConfigs, Plugin, Update};

mod bloom;
mod graphics;
mod wireframe;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowManager::default())
            .insert_resource(BloomSettings::default())
            .insert_resource(GraphicsSettings::default())
            .insert_resource(WireframeSettings::default())
            .add_systems(
                Update,
                apply_bloom_settings.run_if(resource_changed::<BloomSettings>),
            )
            .add_systems(
                Update,
                apply_graphics_settings.run_if(resource_changed::<GraphicsSettings>),
            )
            .add_systems(
                Update,
                apply_wireframe_settings.run_if(resource_changed::<WireframeSettings>),
            );
    }
}
