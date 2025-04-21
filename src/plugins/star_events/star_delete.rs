use crate::events::star_delete::StarDeleteEvent;
use crate::events::star_unselect::StarUnselectEvent;
use crate::resources::selected_star::SelectedStar;
use crate::resources::star_store::StarStore;
use bevy::prelude::{Commands, DespawnRecursiveExt, EventReader, EventWriter, Res, ResMut};

pub fn handle_star_delete(
    mut star_delete_event: EventReader<StarDeleteEvent>,
    mut commands: Commands,
    mut star_store: ResMut<StarStore>,
    selected_star: Res<SelectedStar>,
    mut star_unselect_event: EventWriter<StarUnselectEvent>,
) {
    for event in star_delete_event.read() {
        let id = event.get_id();
        if let Some(entity) = star_store.delete_star(id) {
            commands.entity(entity).despawn_recursive();
        };
        if selected_star.get_id() == Some(id) {
            star_unselect_event.send(StarUnselectEvent);
        }
    }
    star_store.build_sort_indices();
}
