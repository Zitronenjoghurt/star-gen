use crate::plugins::MainPlugins;
use bevy::app::App;
use bevy::color::Color;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::{default, PluginGroup, Window, WindowPlugin};
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::DefaultPlugins;

mod bundles;
mod components;
mod events;
mod physics;
mod plugins;
mod resources;
mod ui;

pub const VERSION_STRING: &str = "pre-alpha";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("Star Generator ({VERSION_STRING})"),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(WireframeConfig {
            global: false,
            default_color: Color::BLACK,
        })
        .add_plugins(MainPlugins)
        .run();
}
