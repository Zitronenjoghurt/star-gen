use crate::events::star_delete_all::StarDeleteAllEvent;
use crate::resources::star_store::StarStore;
use bevy::prelude::{Commands, DespawnRecursiveExt, EventReader, ResMut};

pub fn handle_star_delete_all(
    mut star_delete_all_event: EventReader<StarDeleteAllEvent>,
    mut commands: Commands,
    mut star_store: ResMut<StarStore>,
) {
    let Some(_) = star_delete_all_event.read().last() else {
        return;
    };

    for entity in star_store.delete_all_stars() {
        commands.entity(entity).despawn_recursive();
    }
}
