use bevy::prelude::Event;

#[derive(Debug, Event)]
pub struct CameraTargetRadiusEvent(f32);

impl CameraTargetRadiusEvent {
    pub fn new(radius: f32) -> Self {
        Self(radius)
    }

    pub fn get_target_radius(&self) -> f32 {
        self.0
    }
}
