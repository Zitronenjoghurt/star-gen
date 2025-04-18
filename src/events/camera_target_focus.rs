use bevy::prelude::{Event, GlobalTransform, Vec3};

#[derive(Debug, Event)]
pub struct CameraTargetFocusEvent(Vec3);

impl CameraTargetFocusEvent {
    pub fn from_global_transform(global_transform: &GlobalTransform) -> Self {
        Self(global_transform.translation())
    }

    pub fn get_target_focus_point(&self) -> Vec3 {
        self.0
    }
}
