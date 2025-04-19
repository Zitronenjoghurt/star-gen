use crate::events::star_delete_all::StarDeleteAllEvent;
use crate::events::star_unselect::StarUnselectEvent;
use crate::resources::star_store::StarStore;
use bevy::prelude::{Commands, DespawnRecursiveExt, EventReader, EventWriter, ResMut};

pub fn handle_star_delete_all(
    mut star_delete_all_event: EventReader<StarDeleteAllEvent>,
    mut commands: Commands,
    mut star_store: ResMut<StarStore>,
    mut star_unselect_event: EventWriter<StarUnselectEvent>,
) {
    let Some(_) = star_delete_all_event.read().last() else {
        return;
    };

    for entity in star_store.delete_all_stars() {
        commands.entity(entity).despawn_recursive();
    }

    star_unselect_event.send(StarUnselectEvent);
}
