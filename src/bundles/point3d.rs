use crate::components::selectable_point::SelectablePoint;
use bevy::prelude::{
    default, Bundle, GlobalTransform, Transform, Vec3, ViewVisibility, Visibility,
};
use bevy::render::view::VisibilityRange;

#[derive(Default, Bundle)]
pub struct Point3D {
    selectable_point: SelectablePoint,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    view_visibility: ViewVisibility,
    visibility_range: VisibilityRange,
}

impl Point3D {
    pub fn new(id: u64, position: Vec3, scale: f32, render_distance: f32) -> Self {
        Self {
            selectable_point: SelectablePoint::new(id, false),
            transform: Transform::from_translation(position).with_scale(Vec3::splat(scale)),
            visibility_range: VisibilityRange {
                start_margin: 0.0..0.0,
                end_margin: (render_distance - 1.0)..render_distance,
                ..default()
            },
            ..Default::default()
        }
    }
}
