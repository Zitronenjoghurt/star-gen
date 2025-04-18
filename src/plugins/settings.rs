use crate::plugins::settings::bloom::apply_bloom_settings;
use crate::plugins::settings::wireframe::apply_wireframe_settings;
use crate::resources::simulation_settings::{BloomSettings, WireframeSettings};
use crate::resources::window_manager::WindowManager;
use bevy::prelude::{resource_changed, App, IntoSystemConfigs, Plugin, Update};

mod bloom;
mod wireframe;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowManager::default())
            .insert_resource(WireframeSettings::default())
            .insert_resource(BloomSettings::default())
            .add_systems(
                Update,
                apply_wireframe_settings.run_if(resource_changed::<WireframeSettings>),
            )
            .add_systems(
                Update,
                apply_bloom_settings.run_if(resource_changed::<BloomSettings>),
            );
    }
}
