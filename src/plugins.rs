use crate::plugins::camera_events::CameraEventsPlugin;
use crate::plugins::inputs::InputPlugin;
use crate::plugins::interface::InterfacePlugin;
use crate::plugins::settings::SettingsPlugin;
use crate::plugins::star_events::StarEventsPlugin;
use crate::plugins::startup::StartupPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::PluginGroup;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

pub mod camera_events;
pub mod inputs;
pub mod interface;
mod settings;
pub mod star_events;
pub mod startup;

pub struct MainPlugins;

impl PluginGroup for MainPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FrameTimeDiagnosticsPlugin)
            .add(EntityCountDiagnosticsPlugin)
            .add(RenderDiagnosticsPlugin)
            .add(EguiPlugin)
            .add(InterfacePlugin)
            .add(SettingsPlugin)
            .add(CameraEventsPlugin)
            .add(StarEventsPlugin)
            .add(PanOrbitCameraPlugin)
            .add(StartupPlugin)
            .add(InputPlugin)
    }
}
