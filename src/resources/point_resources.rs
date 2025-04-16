use bevy::prelude::{Handle, Mesh, Resource};

#[derive(Resource)]
pub struct PointResources {
    sphere_mesh: Handle<Mesh>,
}

impl PointResources {
    pub fn new(sphere_mesh: Handle<Mesh>) -> Self {
        Self { sphere_mesh }
    }

    pub fn sphere_mesh(&self) -> Handle<Mesh> {
        self.sphere_mesh.clone()
    }
}
