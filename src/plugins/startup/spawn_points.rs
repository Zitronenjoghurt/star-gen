use crate::events::star_spawn::StarSpawnEvent;
use crate::resources::star_store::StarStore;
use bevy::prelude::{EventWriter, ResMut, Vec3};
use rand::Rng;

const TOTAL_SPREAD: f32 = 100.0;
const CUBIC_COUNT: i32 = 4;
const OFFSET_FACTOR: f32 = 10.0;

pub fn spawn_points(
    mut star_spawn_event: EventWriter<StarSpawnEvent>,
    mut star_store: ResMut<StarStore>,
) {
    let rng = star_store.get_rng();
    for x in -CUBIC_COUNT..=CUBIC_COUNT {
        for y in -CUBIC_COUNT..=CUBIC_COUNT {
            for z in -CUBIC_COUNT..=CUBIC_COUNT {
                let offset_x = rng.random::<f32>() * OFFSET_FACTOR - 1.0;
                let offset_y = rng.random::<f32>() * OFFSET_FACTOR - 1.0;
                let offset_z = rng.random::<f32>() * OFFSET_FACTOR - 1.0;

                let position = Vec3::new(
                    (x as f32 + offset_x) * TOTAL_SPREAD,
                    (y as f32 + offset_y) * TOTAL_SPREAD,
                    (z as f32 + offset_z) * TOTAL_SPREAD,
                );

                star_spawn_event.send(StarSpawnEvent::random(rng, position));
            }
        }
    }
}
