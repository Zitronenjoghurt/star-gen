use crate::events::star_cluster_generate::StarClusterGenerateEvent;
use crate::events::star_delete_all::StarDeleteAllEvent;
use crate::events::star_spawn::StarSpawnEvent;
use crate::resources::star_store::StarStore;
use bevy::prelude::{EventReader, EventWriter, ResMut};

pub fn handle_star_cluster_generate(
    mut star_cluster_generate_event: EventReader<StarClusterGenerateEvent>,
    mut star_delete_all_event: EventWriter<StarDeleteAllEvent>,
    mut star_spawn_event: EventWriter<StarSpawnEvent>,
    mut star_store: ResMut<StarStore>,
) {
    let Some(event) = star_cluster_generate_event.read().last() else {
        return;
    };

    star_delete_all_event.send(StarDeleteAllEvent);

    let method = event.get_method();
    method.generate(&mut star_store, &mut star_spawn_event);
}
