use crate::utils::misc::clamp_range;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Resource, PartialEq, Serialize, Deserialize)]
pub struct BloomSettings {
    pub active: bool,
    pub intensity: f32,
    pub low_frequency_boost: f32,
    pub low_frequency_boost_curvature: f32,
    pub high_pass_frequency: f32,
    pub prefilter_threshold: f32,
    pub prefilter_threshold_softness: f32,
}

impl Default for BloomSettings {
    fn default() -> Self {
        Self {
            active: Self::DEFAULT_ACTIVE,
            intensity: Self::DEFAULT_INTENSITY,
            low_frequency_boost: Self::DEFAULT_LOW_FREQUENCY_BOOST,
            low_frequency_boost_curvature: Self::DEFAULT_LOW_FREQUENCY_BOOST_CURVATURE,
            high_pass_frequency: Self::DEFAULT_HIGH_PASS_FREQUENCY,
            prefilter_threshold: Self::DEFAULT_PREFILTER_THRESHOLD,
            prefilter_threshold_softness: Self::DEFAULT_PREFILTER_THRESHOLD_SOFTNESS,
        }
    }
}

impl BloomSettings {
    pub const DEFAULT_ACTIVE: bool = true;
    pub const DEFAULT_INTENSITY: f32 = 0.25;
    pub const DEFAULT_LOW_FREQUENCY_BOOST: f32 = 0.5;
    pub const DEFAULT_LOW_FREQUENCY_BOOST_CURVATURE: f32 = 0.4;
    pub const DEFAULT_HIGH_PASS_FREQUENCY: f32 = 0.8;
    pub const DEFAULT_PREFILTER_THRESHOLD: f32 = 0.4;
    pub const DEFAULT_PREFILTER_THRESHOLD_SOFTNESS: f32 = 0.1;
    pub const RANGE_INTENSITY: RangeInclusive<f32> = 0.0..=2.0;
    pub const RANGE_LOW_FREQUENCY_BOOST: RangeInclusive<f32> = 0.0..=1.0;
    pub const RANGE_LOW_FREQUENCY_BOOST_CURVATURE: RangeInclusive<f32> = 0.0..=1.0;
    pub const RANGE_HIGH_PASS_FREQUENCY: RangeInclusive<f32> = 0.0..=1.0;
    pub const RANGE_PREFILTER_THRESHOLD: RangeInclusive<f32> = 0.0..=50.0;
    pub const RANGE_PREFILTER_THRESHOLD_SOFTNESS: RangeInclusive<f32> = 0.0..=1.0;

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn apply_validation(&mut self) {
        clamp_range(&mut self.intensity, &Self::RANGE_INTENSITY);
        clamp_range(
            &mut self.low_frequency_boost,
            &Self::RANGE_LOW_FREQUENCY_BOOST,
        );
        clamp_range(
            &mut self.low_frequency_boost_curvature,
            &Self::RANGE_LOW_FREQUENCY_BOOST,
        );
        clamp_range(
            &mut self.high_pass_frequency,
            &Self::RANGE_HIGH_PASS_FREQUENCY,
        );
        clamp_range(
            &mut self.prefilter_threshold,
            &Self::RANGE_PREFILTER_THRESHOLD,
        );
        clamp_range(
            &mut self.prefilter_threshold_softness,
            &Self::RANGE_PREFILTER_THRESHOLD_SOFTNESS,
        );
    }
}
