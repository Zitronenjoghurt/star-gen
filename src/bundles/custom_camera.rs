use crate::resources::settings::bloom::BloomSettings;
use bevy::core_pipeline::bloom::{Bloom, BloomCompositeMode, BloomPrefilter};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::math::Vec3;
use bevy::prelude::{default, Bundle, Camera, Camera3d, MouseButton, Transform};
use bevy_panorbit_camera::PanOrbitCamera;

#[derive(Bundle)]
pub struct CustomCamera {
    camera3d: Camera3d,
    camera: Camera,
    transform: Transform,
    pan_orbit: PanOrbitCamera,
    tonemapping: Tonemapping,
    bloom: Bloom,
}

impl Default for CustomCamera {
    fn default() -> Self {
        Self {
            camera3d: Camera3d::default(),
            camera: Camera {
                hdr: BloomSettings::DEFAULT_ACTIVE,
                ..default()
            },
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            pan_orbit: PanOrbitCamera {
                button_orbit: MouseButton::Right,
                button_pan: MouseButton::Middle,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            bloom: Bloom {
                intensity: BloomSettings::DEFAULT_INTENSITY,
                low_frequency_boost: BloomSettings::DEFAULT_LOW_FREQUENCY_BOOST,
                low_frequency_boost_curvature: BloomSettings::DEFAULT_LOW_FREQUENCY_BOOST_CURVATURE,
                high_pass_frequency: BloomSettings::DEFAULT_HIGH_PASS_FREQUENCY,
                prefilter: BloomPrefilter {
                    threshold: BloomSettings::DEFAULT_PREFILTER_THRESHOLD,
                    threshold_softness: BloomSettings::DEFAULT_PREFILTER_THRESHOLD_SOFTNESS,
                },
                composite_mode: BloomCompositeMode::Additive,
                ..default()
            },
        }
    }
}
