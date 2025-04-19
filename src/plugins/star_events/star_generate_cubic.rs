use crate::events::star_generate_cubic::StarGenerateCubicEvent;
use crate::events::star_spawn::StarSpawnEvent;
use crate::resources::star_store::StarStore;
use bevy::math::Vec3;
use bevy::prelude::{EventReader, EventWriter, ResMut};
use rand::Rng;

pub fn handle_star_generate_cubic(
    mut star_generate_cubic_event: EventReader<StarGenerateCubicEvent>,
    mut star_spawn_event: EventWriter<StarSpawnEvent>,
    mut star_store: ResMut<StarStore>,
) {
    let Some(event) = star_generate_cubic_event.read().last() else {
        return;
    };

    star_store.set_rng_seed(event.seed);
    let rng = star_store.get_rng();
    for x in event.min_x..=event.max_x {
        for y in event.min_y..=event.max_y {
            for z in event.min_z..=event.max_z {
                let offset_x = rng.random::<f32>() * event.offset_factor - 1.0;
                let offset_y = rng.random::<f32>() * event.offset_factor - 1.0;
                let offset_z = rng.random::<f32>() * event.offset_factor - 1.0;

                let position = Vec3::new(
                    (x as f32 + offset_x) * event.spread,
                    (y as f32 + offset_y) * event.spread,
                    (z as f32 + offset_z) * event.spread,
                );

                star_spawn_event.send(StarSpawnEvent::random(rng, position));
            }
        }
    }
}
