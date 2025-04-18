use crate::resources::simulation_settings::BloomSettings;
use bevy::prelude::{Camera, Query, Res};

pub fn apply_bloom_settings(bloom_settings: Res<BloomSettings>, mut camera: Query<&mut Camera>) {
    for mut camera in camera.iter_mut() {
        camera.hdr = bloom_settings.active;
    }
}
