use crate::events::camera_target_focus::CameraTargetFocusEvent;
use crate::plugins::camera_events::target_focus::handle_camera_target_focus;
use bevy::prelude::{App, Plugin, Update};

mod target_focus;

pub struct CameraEventsPlugin;

impl Plugin for CameraEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_camera_target_focus)
            .add_event::<CameraTargetFocusEvent>();
    }
}
