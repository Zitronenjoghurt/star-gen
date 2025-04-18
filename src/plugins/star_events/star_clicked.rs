use crate::components::selectable_point::SelectablePoint;
use crate::components::star_data::StarData;
use crate::events::camera_target_focus::CameraTargetFocusEvent;
use crate::events::star_clicked::StarClickedEvent;
use crate::resources::selected_star::SelectedStar;
use bevy::prelude::{EventReader, EventWriter, GlobalTransform, Query, ResMut};

pub fn handle_star_clicked(
    mut star_clicked_event: EventReader<StarClickedEvent>,
    mut camera_target_focus_event: EventWriter<CameraTargetFocusEvent>,
    mut selected_star: ResMut<SelectedStar>,
    mut selectable_points: Query<(&mut SelectablePoint, &GlobalTransform, &StarData)>,
) {
    let Some(event) = star_clicked_event.read().last() else {
        return;
    };

    let previous_id = selected_star.get_id();
    let target_id = event.get_id();
    if previous_id == Some(target_id) {
        return;
    }

    selected_star.set_id(target_id);

    let mut selected = false;
    let mut unselected = false;
    for (mut selectable_point, transform, data) in selectable_points.iter_mut() {
        if Some(selectable_point.get_id()) == previous_id {
            unselected = true;
            selectable_point.unselect();
        } else if selectable_point.get_id() == target_id {
            selected = true;
            selectable_point.select();
            selected_star.set_data(data.get_star().clone());
            selected_star.set_position(transform.translation());
            camera_target_focus_event
                .send(CameraTargetFocusEvent::from_global_transform(transform));
        }

        if (selected && unselected) {
            break;
        }
    }
}
