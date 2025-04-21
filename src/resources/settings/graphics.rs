use crate::utils::misc::clamp_range;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub vsync: bool,
    pub render_distance: f32,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            vsync: true,
            render_distance: Self::DEFAULT_RENDER_DISTANCE,
        }
    }
}

impl GraphicsSettings {
    pub const DEFAULT_RENDER_DISTANCE: f32 = 10000.0;
    pub const RANGE_RENDER_DISTANCE: RangeInclusive<f32> = 0.0..=10000.0;

    pub fn toggle_vsync(&mut self) {
        self.vsync = !self.vsync;
    }

    pub fn apply_validations(&mut self) {
        clamp_range(&mut self.render_distance, &Self::RANGE_RENDER_DISTANCE);
    }
}
