use crate::events::star_unselect::StarUnselectEvent;
use crate::resources::selected_star::SelectedStar;
use bevy::prelude::{EventReader, ResMut};

pub fn handle_star_unselect(
    mut star_unselect_event: EventReader<StarUnselectEvent>,
    mut selected_star: ResMut<SelectedStar>,
) {
    let Some(_) = star_unselect_event.read().last() else {
        return;
    };
    selected_star.clear();
}
