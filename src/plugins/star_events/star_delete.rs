use crate::events::star_delete::StarDeleteEvent;
use crate::resources::star_store::StarStore;
use bevy::prelude::{Commands, DespawnRecursiveExt, EventReader, ResMut};

pub fn handle_star_delete(
    mut star_delete_event: EventReader<StarDeleteEvent>,
    mut commands: Commands,
    mut star_store: ResMut<StarStore>,
) {
    for event in star_delete_event.read() {
        let id = event.get_id();
        if let Some(entity) = star_store.delete_star(id) {
            commands.entity(entity).despawn_recursive();
        };
    }
}
