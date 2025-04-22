use crate::utils::misc::clamp_range;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Resource, PartialEq, Serialize, Deserialize)]
pub struct ControlSettings {
    pub star_focus_auto_zoom_factor: f32,
}

impl Default for ControlSettings {
    fn default() -> Self {
        Self {
            star_focus_auto_zoom_factor: Self::DEFAULT_STAR_FOCUS_AUTO_ZOOM_FACTOR,
        }
    }
}

impl ControlSettings {
    pub const DEFAULT_STAR_FOCUS_AUTO_ZOOM_FACTOR: f32 = 7.0;
    pub const RANGE_STAR_FOCUS_AUTO_ZOOM_FACTOR: RangeInclusive<f32> = 1.0..=20.0;

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn apply_validations(&mut self) {
        clamp_range(
            &mut self.star_focus_auto_zoom_factor,
            &Self::RANGE_STAR_FOCUS_AUTO_ZOOM_FACTOR,
        );
    }
}
