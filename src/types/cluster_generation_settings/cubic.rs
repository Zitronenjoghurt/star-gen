use crate::events::star_spawn::StarSpawnEvent;
use crate::resources::star_store::StarStore;
use crate::utils::base64::{b64_decode_bytes, b64_encode_bytes};
use bevy::math::Vec3;
use bevy::prelude::{EventWriter, ResMut};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct CubicClusterGenerationSettings {
    pub min_x: i8,
    pub max_x: i8,
    pub min_y: i8,
    pub max_y: i8,
    pub min_z: i8,
    pub max_z: i8,
    pub spread: f32,
    pub offset_factor: f32,
    pub seed: u64,
}

impl CubicClusterGenerationSettings {
    pub const DEFAULT_MIN_X: i8 = -4;
    pub const DEFAULT_MAX_X: i8 = 4;
    pub const DEFAULT_MIN_Y: i8 = -4;
    pub const DEFAULT_MAX_Y: i8 = 4;
    pub const DEFAULT_MIN_Z: i8 = -4;
    pub const DEFAULT_MAX_Z: i8 = 4;
    pub const DEFAULT_SPREAD: f32 = 100.0;
    pub const DEFAULT_OFFSET_FACTOR: f32 = 10.0;

    pub fn default_with_seed(seed: u64) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }

    pub fn encode(&self) -> String {
        let mut bytes = vec![0u8; 22];
        bytes[0] = self.min_x as u8;
        bytes[1] = self.max_x as u8;
        bytes[2] = self.min_y as u8;
        bytes[3] = self.max_y as u8;
        bytes[4] = self.min_z as u8;
        bytes[5] = self.max_z as u8;
        bytes[6..10].copy_from_slice(&self.spread.to_le_bytes());
        bytes[10..14].copy_from_slice(&self.offset_factor.to_le_bytes());
        bytes[14..22].copy_from_slice(&self.seed.to_le_bytes());
        let encoded = b64_encode_bytes(bytes.as_slice());
        format!("cubic:{encoded}")
    }

    pub fn decode(encoded: &str) -> Option<Self> {
        let encoded_b64 = encoded.strip_prefix("cubic:")?;
        let bytes = b64_decode_bytes(encoded_b64)?;
        if bytes.len() != 22 {
            return None;
        }

        Some(Self {
            min_x: bytes[0] as i8,
            max_x: bytes[1] as i8,
            min_y: bytes[2] as i8,
            max_y: bytes[3] as i8,
            min_z: bytes[4] as i8,
            max_z: bytes[5] as i8,
            spread: f32::from_le_bytes(bytes[6..10].try_into().ok()?),
            offset_factor: f32::from_le_bytes(bytes[10..14].try_into().ok()?),
            seed: u64::from_le_bytes(bytes[14..22].try_into().ok()?),
        })
    }

    pub fn generate(
        &self,
        star_store: &mut ResMut<StarStore>,
        star_spawn_event: &mut EventWriter<StarSpawnEvent>,
    ) {
        let rng = star_store.get_rng();
        for x in self.min_x..self.max_x {
            for y in self.min_y..self.max_y {
                for z in self.min_z..self.max_z {
                    let offset_x = rng.random::<f32>() * self.offset_factor - 1.0;
                    let offset_y = rng.random::<f32>() * self.offset_factor - 1.0;
                    let offset_z = rng.random::<f32>() * self.offset_factor - 1.0;

                    let position = Vec3::new(
                        (x as f32 + offset_x) * self.spread,
                        (y as f32 + offset_y) * self.spread,
                        (z as f32 + offset_z) * self.spread,
                    );

                    star_spawn_event.send(StarSpawnEvent::random(rng, position));
                }
            }
        }
    }
}

impl Default for CubicClusterGenerationSettings {
    fn default() -> Self {
        Self {
            min_x: Self::DEFAULT_MIN_X,
            max_x: Self::DEFAULT_MAX_X,
            min_y: Self::DEFAULT_MIN_Y,
            max_y: Self::DEFAULT_MAX_Y,
            min_z: Self::DEFAULT_MIN_Z,
            max_z: Self::DEFAULT_MAX_Z,
            spread: Self::DEFAULT_SPREAD,
            offset_factor: Self::DEFAULT_OFFSET_FACTOR,
            seed: 0,
        }
    }
}
