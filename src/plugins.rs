use crate::plugins::camera_events::CameraEventsPlugin;
use crate::plugins::inputs::InputPlugin;
use crate::plugins::interface::InterfacePlugin;
use crate::plugins::misc::MiscPlugin;
use crate::plugins::settings::SettingsPlugin;
use crate::plugins::star_events::StarEventsPlugin;
use crate::plugins::system::SystemPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::PluginGroup;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

pub mod camera_events;
pub mod inputs;
pub mod interface;
mod misc;
mod settings;
pub mod star_events;
pub mod system;

pub struct MainPlugins;

impl PluginGroup for MainPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EguiPlugin)
            .add(EntityCountDiagnosticsPlugin)
            .add(FrameTimeDiagnosticsPlugin)
            .add(PanOrbitCameraPlugin)
            .add(RenderDiagnosticsPlugin)
            .add(WireframePlugin)
            .add(CameraEventsPlugin)
            .add(InputPlugin)
            .add(InterfacePlugin)
            .add(MiscPlugin)
            .add(SettingsPlugin)
            .add(StarEventsPlugin)
            .add(SystemPlugin)
    }
}
