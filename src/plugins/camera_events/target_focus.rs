use crate::events::camera_target_focus::CameraTargetFocusEvent;
use bevy::prelude::{EventReader, Query};
use bevy_panorbit_camera::PanOrbitCamera;

pub fn handle_camera_target_focus(
    mut camera_target_focus_event: EventReader<CameraTargetFocusEvent>,
    mut pan_orbit_camera: Query<&mut PanOrbitCamera>,
) {
    let Some(event) = camera_target_focus_event.read().last() else {
        return;
    };

    let mut camera = pan_orbit_camera.single_mut();
    camera.target_focus = event.get_target_focus_point();
}
