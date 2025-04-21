use crate::plugins::MainPlugins;
use bevy::app::App;
use bevy::color::Color;
use bevy::core::TaskPoolThreadAssignmentPolicy;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::{default, PluginGroup, TaskPoolOptions, TaskPoolPlugin, Window, WindowPlugin};
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::tasks::available_parallelism;
use bevy::DefaultPlugins;

mod bundles;
mod components;
mod events;
mod physics;
mod plugins;
mod resources;
mod types;
mod ui;
mod utils;

pub const VERSION_STRING: &str = "v0.0.2";

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
                })
                // No parallelized I/O threads, using more threads for computing, potentially less idling but potential performance implications on I/O operations
                // https://bevy-cheatbook.github.io/setup/perf.html#overprovisioning
                .set(TaskPoolPlugin {
                    task_pool_options: TaskPoolOptions {
                        compute: TaskPoolThreadAssignmentPolicy {
                            min_threads: available_parallelism(),
                            max_threads: usize::MAX,
                            percent: 1.0,
                        },
                        ..default()
                    },
                }),
        )
        .insert_resource(WireframeConfig {
            global: false,
            default_color: Color::BLACK,
        })
        .add_plugins(MainPlugins)
        .run();
}
