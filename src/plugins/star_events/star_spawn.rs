use crate::bundles::star::StarBundle;
use crate::events::star_spawn::StarSpawnEvent;
use crate::materials::star::StarMaterial;
use crate::resources::settings::graphics::GraphicsSettings;
use crate::resources::star_store::StarStore;
use bevy::asset::Assets;
use bevy::prelude::{Commands, EventReader, Mesh, Res, ResMut};

pub fn handle_star_spawn(
    mut star_spawn_event: EventReader<StarSpawnEvent>,
    mut commands: Commands,
    mut star_store: ResMut<StarStore>,
    mut materials: ResMut<Assets<StarMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    graphics_settings: Res<GraphicsSettings>,
) {
    for event in star_spawn_event.read() {
        let star = event.get_star().clone();

        let id = if let Some(id) = event.get_id() {
            star_store.add_star_with_id(star.clone(), id);
            id
        } else {
            star_store.add_star(star.clone())
        };

        let bundle = StarBundle::new(
            star,
            id,
            &mut meshes,
            &mut materials,
            graphics_settings.render_distance,
        );
        let entity = commands.spawn(bundle).id();
        star_store.add_entity(id, entity);
    }
    star_store.build_sort_indices();
}
