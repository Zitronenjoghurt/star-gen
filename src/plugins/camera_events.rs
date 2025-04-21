use crate::events::camera_target_focus::CameraTargetFocusEvent;
use crate::events::camera_target_radius::CameraTargetRadiusEvent;
use crate::plugins::camera_events::target_focus::handle_camera_target_focus;
use crate::plugins::camera_events::target_radius::handle_camera_target_radius;
use crate::plugins::misc::observe_pan_orbit::observe_pan_orbit_camera;
use bevy::prelude::{on_event, App, IntoSystemConfigs, Plugin, Update};

mod target_focus;
mod target_radius;

pub struct CameraEventsPlugin;

impl Plugin for CameraEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_camera_target_focus
                    .run_if(on_event::<CameraTargetFocusEvent>)
                    .before(observe_pan_orbit_camera),
                handle_camera_target_radius.run_if(on_event::<CameraTargetRadiusEvent>),
            ),
        )
        .add_event::<CameraTargetFocusEvent>()
        .add_event::<CameraTargetRadiusEvent>();
    }
}
