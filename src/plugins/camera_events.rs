use crate::events::camera_target_focus::CameraTargetFocusEvent;
use crate::plugins::camera_events::target_focus::handle_camera_target_focus;
use crate::plugins::misc::observe_pan_orbit::observe_pan_orbit_camera;
use bevy::prelude::{App, IntoSystemConfigs, Plugin, Update};

mod target_focus;

pub struct CameraEventsPlugin;

impl Plugin for CameraEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_camera_target_focus.before(observe_pan_orbit_camera),
        )
        .add_event::<CameraTargetFocusEvent>();
    }
}
