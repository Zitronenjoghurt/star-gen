use crate::plugins::startup::StartupPlugin;
use bevy::app::App;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::{Color, PluginGroup};
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod bundles;
mod components;
mod plugins;
mod resources;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            WireframePlugin,
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::BLACK,
        })
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(StartupPlugin)
        .run();
}
