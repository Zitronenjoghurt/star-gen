use crate::events::star_spawn::StarSpawnEvent;
use crate::physics::objects::star::Star;
use bevy::prelude::{Events, Vec3, World};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StarState {
    id: u64,
    x: f32,
    y: f32,
    z: f32,
    mass: f64,
}

impl StarState {
    pub fn save(id: u64, star: &Star) -> Self {
        let position = star.get_position();
        Self {
            id,
            x: position.x,
            y: position.y,
            z: position.z,
            mass: star.get_mass(),
        }
    }

    pub fn load(&self, world: &mut World) {
        let position = Vec3::new(self.x, self.y, self.z);
        let star = Star::new(self.mass, position);
        if let Some(mut star_spawn_event) = world.get_resource_mut::<Events<StarSpawnEvent>>() {
            star_spawn_event.send(StarSpawnEvent::new(star, Some(self.id)));
        };
    }
}
