use bevy::prelude::{Asset, LinearRgba, Material, TypePath};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

const FRAGMENT_SHADER_PATH: &str = "shaders/star.wgsl";

#[derive(Debug, Clone, Asset, AsBindGroup, TypePath)]
pub struct StarMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material for StarMaterial {
    fn fragment_shader() -> ShaderRef {
        FRAGMENT_SHADER_PATH.into()
    }
}
