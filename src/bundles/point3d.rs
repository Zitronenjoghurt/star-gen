use crate::components::selectable_point::SelectablePoint;
use bevy::prelude::{
    default, Bundle, GlobalTransform, Handle, InheritedVisibility, Mesh, Mesh3d,
    MeshMaterial3d, StandardMaterial, Transform, Vec3, ViewVisibility, Visibility,
};
use bevy::render::view::VisibilityRange;

#[derive(Default, Bundle)]
pub struct Point3D {
    selectable_point: SelectablePoint,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,
    visibility_range: VisibilityRange,
}

impl Point3D {
    pub fn new(
        id: u64,
        position: Vec3,
        scale: f32,
        mesh_handle: Handle<Mesh>,
        material_handle: Handle<StandardMaterial>,
        render_distance: f32,
    ) -> Self {
        Self {
            selectable_point: SelectablePoint::new(id, false),
            mesh: Mesh3d(mesh_handle),
            material: MeshMaterial3d(material_handle),
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
