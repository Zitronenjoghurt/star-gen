use bevy::prelude::Event;

#[derive(Debug, Default, Event)]
pub struct StarGenerateCubicEvent {
    pub min_x: i64,
    pub max_x: i64,
    pub min_y: i64,
    pub max_y: i64,
    pub min_z: i64,
    pub max_z: i64,
    pub spread: f32,
    pub offset_factor: f32,
    pub seed: u64,
}
