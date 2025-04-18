use bevy::core_pipeline::bloom::{Bloom, BloomCompositeMode, BloomPrefilter};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::math::Vec3;
use bevy::prelude::{default, Bundle, Camera, Camera3d, Transform};
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
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            pan_orbit: PanOrbitCamera::default(),
            tonemapping: Tonemapping::TonyMcMapface,
            bloom: Bloom {
                intensity: 0.25,
                low_frequency_boost: 0.5,
                low_frequency_boost_curvature: 0.95,
                high_pass_frequency: 0.8,
                prefilter: BloomPrefilter {
                    threshold: 0.4,
                    threshold_softness: 0.1,
                },
                composite_mode: BloomCompositeMode::Additive,
                ..default()
            },
        }
    }
}
