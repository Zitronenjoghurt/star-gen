use bevy::prelude::Resource;

#[derive(Debug, Resource)]
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

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
