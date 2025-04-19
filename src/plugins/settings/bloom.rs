use crate::resources::settings::bloom::BloomSettings;
use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::{Camera, Query, Res};

pub fn apply_bloom_settings(
    bloom_settings: Res<BloomSettings>,
    mut camera: Query<(&mut Camera, &mut Bloom)>,
) {
    for (mut camera, mut bloom) in camera.iter_mut() {
        camera.hdr = bloom_settings.active;
        bloom.intensity = bloom_settings.intensity;
        bloom.low_frequency_boost = bloom_settings.low_frequency_boost;
        bloom.low_frequency_boost_curvature = bloom_settings.low_frequency_boost_curvature;
        bloom.high_pass_frequency = bloom_settings.high_pass_frequency;
        bloom.prefilter.threshold = bloom_settings.prefilter_threshold;
        bloom.prefilter.threshold_softness = bloom_settings.prefilter_threshold_softness;
    }
}
