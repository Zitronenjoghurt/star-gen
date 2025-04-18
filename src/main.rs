use crate::plugins::MainPlugins;
use bevy::app::App;
use bevy::color::Color;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::{default, PluginGroup};
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::DefaultPlugins;

mod bundles;
mod components;
mod events;
mod physics;
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
            global: false,
            default_color: Color::BLACK,
        })
        .add_plugins(MainPlugins)
        .run();
}
