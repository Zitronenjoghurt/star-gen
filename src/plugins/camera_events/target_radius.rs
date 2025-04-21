use crate::events::camera_target_radius::CameraTargetRadiusEvent;
use bevy::prelude::{EventReader, Query};
use bevy_panorbit_camera::PanOrbitCamera;

pub fn handle_camera_target_radius(
    mut camera_target_radius_event: EventReader<CameraTargetRadiusEvent>,
    mut pan_orbit_camera: Query<&mut PanOrbitCamera>,
) {
    let Some(event) = camera_target_radius_event.read().last() else {
        return;
    };

    let mut camera = pan_orbit_camera.single_mut();
    camera.target_radius = event.get_target_radius();
}
