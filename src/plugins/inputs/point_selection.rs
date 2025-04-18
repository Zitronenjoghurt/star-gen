use crate::components::selectable_point::SelectablePoint;
use crate::events::star_clicked::StarClickedEvent;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn point_selection(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut selectable_points: Query<(&mut SelectablePoint, &mut Transform, &GlobalTransform)>,
    mut star_clicked_event: EventWriter<StarClickedEvent>,
) {
    if !mouse_input.just_released(MouseButton::Middle) {
        return;
    }

    let (camera, camera_transform) = cameras.single();

    let main_window = windows.single();
    let Some(cursor_position) = main_window.cursor_position() else {
        return;
    };

    let mut closest_point: Option<u64> = None;
    let mut min_distance = f32::MAX;

    for (point, _, global_transform) in selectable_points.iter_mut() {
        let world_position = global_transform.translation();

        let view_matrix = camera_transform.compute_matrix().inverse();
        let view_position = view_matrix * world_position.extend(1.0);

        if view_position.z > 0.0 {
            continue;
        }

        let Ok(screen_position) = camera.world_to_viewport(camera_transform, world_position) else {
            continue;
        };

        if screen_position.x >= 0.0
            && screen_position.x <= main_window.width()
            && screen_position.y >= 0.0
            && screen_position.y <= main_window.height()
        {
            let distance_squared = (screen_position - cursor_position).length_squared();
            let scaled_distance = distance_squared * (1.0 + (-view_position.z) * 0.01);

            if scaled_distance < min_distance {
                min_distance = scaled_distance;
                closest_point = Some(point.get_id());
            }
        }
    }

    if let Some(closest_id) = closest_point {
        star_clicked_event.send(StarClickedEvent::new(closest_id));
    }
}
