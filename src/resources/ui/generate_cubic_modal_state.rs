use crate::events::star_generate_cubic::StarGenerateCubicEvent;
use crate::utils::base64::{b64_decode_u64, b64_encode_u64};
use crate::utils::random::random_u64;
use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct GenerateCubicModalState {
    pub min_x: i64,
    pub max_x: i64,
    pub min_y: i64,
    pub max_y: i64,
    pub min_z: i64,
    pub max_z: i64,
    pub spread: f32,
    pub offset_factor: f32,
    pub seed: String,
    pub edit_seed: String,
}

impl GenerateCubicModalState {
    pub const DEFAULT_MIN_X: i64 = -4;
    pub const DEFAULT_MAX_X: i64 = 4;
    pub const DEFAULT_MIN_Y: i64 = -4;
    pub const DEFAULT_MAX_Y: i64 = 4;
    pub const DEFAULT_MIN_Z: i64 = -4;
    pub const DEFAULT_MAX_Z: i64 = 4;
    pub const DEFAULT_SPREAD: f32 = 100.0;
    pub const DEFAULT_OFFSET_FACTOR: f32 = 10.0;

    pub fn regenerate_seed(&mut self) {
        self.set_seed(b64_encode_u64(random_u64()));
    }

    pub fn set_seed(&mut self, seed: String) {
        if b64_decode_u64(&seed).is_some() {
            self.seed = seed.clone();
            self.edit_seed = seed;
        } else {
            self.edit_seed = self.seed.clone();
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn get_event(&self) -> Option<StarGenerateCubicEvent> {
        let seed = b64_decode_u64(&self.seed)?;

        Some(StarGenerateCubicEvent {
            min_x: self.min_x,
            max_x: self.max_x,
            min_y: self.min_y,
            max_y: self.max_y,
            min_z: self.min_z,
            max_z: self.max_z,
            spread: self.spread,
            offset_factor: self.offset_factor,
            seed,
        })
    }
}

impl Default for GenerateCubicModalState {
    fn default() -> Self {
        let seed = b64_encode_u64(random_u64());
        Self {
            min_x: Self::DEFAULT_MIN_X,
            max_x: Self::DEFAULT_MAX_X,
            min_y: Self::DEFAULT_MIN_Y,
            max_y: Self::DEFAULT_MAX_Y,
            min_z: Self::DEFAULT_MIN_Z,
            max_z: Self::DEFAULT_MAX_Z,
            spread: Self::DEFAULT_SPREAD,
            offset_factor: Self::DEFAULT_OFFSET_FACTOR,
            seed: seed.clone(),
            edit_seed: seed,
        }
    }
}
