use crate::bundles::star::StarBundle;
use crate::events::star_spawn::StarSpawnEvent;
use crate::resources::star_store::StarStore;
use bevy::asset::Assets;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Commands, EventReader, Mesh, ResMut};

pub fn handle_star_spawn(
    mut star_spawn_event: EventReader<StarSpawnEvent>,
    mut commands: Commands,
    mut star_store: ResMut<StarStore>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in star_spawn_event.read() {
        let star = event.get_star().clone();
        let id = star_store.add_star(star.clone());
        let bundle = StarBundle::new(star, id, &mut meshes, &mut materials);
        let entity = commands.spawn(bundle).id();
        star_store.add_entity(id, entity);
    }
}
