use crate::components::selectable_point::SelectablePoint;
use crate::components::star_data::StarData;
use crate::events::camera_target_focus::CameraTargetFocusEvent;
use crate::events::star_clicked::StarClickedEvent;
use crate::resources::selected_star::SelectedStar;
use crate::resources::star_store::StarStore;
use bevy::prelude::{EventReader, EventWriter, GlobalTransform, Query, Res, ResMut};

pub fn handle_star_clicked(
    mut star_clicked_event: EventReader<StarClickedEvent>,
    mut camera_target_focus_event: EventWriter<CameraTargetFocusEvent>,
    mut selected_star: ResMut<SelectedStar>,
    mut selectable_stars: Query<(&mut SelectablePoint, &GlobalTransform, &StarData)>,
    star_store: Res<StarStore>,
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

    if let Some(prev_id) = previous_id {
        if let Some(entity) = star_store.get_entity(prev_id) {
            if let Ok((mut selectable_point, _, _)) = selectable_stars.get_mut(entity) {
                selectable_point.unselect();
            }
        }
    }

    if let Some(entity) = star_store.get_entity(target_id) {
        if let Ok((mut selectable_point, transform, data)) = selectable_stars.get_mut(entity) {
            selectable_point.select();
            selected_star.set_data(data.get_star().clone());
            selected_star.set_position(transform.translation());
            camera_target_focus_event
                .send(CameraTargetFocusEvent::from_global_transform(transform));
        }
    }
}
