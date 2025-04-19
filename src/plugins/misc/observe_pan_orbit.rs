use crate::events::star_unselect::StarUnselectEvent;
use crate::resources::selected_star::SelectedStar;
use bevy::prelude::{Changed, EventWriter, Query, Res};
use bevy_panorbit_camera::PanOrbitCamera;

pub fn observe_pan_orbit_camera(
    pan_orbit: Query<&PanOrbitCamera, Changed<PanOrbitCamera>>,
    selected_star: Res<SelectedStar>,
    mut unselect_star_event: EventWriter<StarUnselectEvent>,
) {
    let Ok(camera) = pan_orbit.get_single() else {
        return;
    };

    if selected_star.get_id().is_some() && Some(camera.target_focus) != selected_star.get_position()
    {
        unselect_star_event.send(StarUnselectEvent);
    }
}
