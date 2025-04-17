use bevy::asset::Assets;
use bevy::color::LinearRgba;
use bevy::prelude::{Color, Handle, Mesh, ResMut, Resource, StandardMaterial};
use std::collections::HashMap;

#[derive(Resource)]
pub struct PointResources {
    sphere_mesh: Handle<Mesh>,
    materials: HashMap<PointMaterialKey, Handle<StandardMaterial>>,
}

impl PointResources {
    pub fn new(sphere_mesh: Handle<Mesh>) -> Self {
        Self {
            sphere_mesh,
            materials: HashMap::new(),
        }
    }

    pub fn sphere_mesh(&self) -> Handle<Mesh> {
        self.sphere_mesh.clone()
    }

    pub fn get_material(
        &mut self,
        red: f32,
        green: f32,
        blue: f32,
        luminance: f32,
        assets: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Handle<StandardMaterial> {
        let color = Color::linear_rgb(red, green, blue);
        let key = PointMaterialKey::from_color_and_luminance(color, luminance);
        if let Some(material) = self.materials.get(&key) {
            material.clone()
        } else {
            let material_handle = Self::create_new_material(assets, color, luminance);
            self.materials.insert(key, material_handle.clone());
            material_handle
        }
    }

    fn create_new_material(
        assets: &mut ResMut<Assets<StandardMaterial>>,
        color: Color,
        luminance: f32,
    ) -> Handle<StandardMaterial> {
        let rgba = color.to_linear();
        assets.add(StandardMaterial {
            base_color: color,
            emissive: LinearRgba::rgb(
                rgba.red * luminance,
                rgba.green * luminance,
                rgba.blue * luminance,
            ),
            metallic: 0.0,
            reflectance: 0.0,
            ..Default::default()
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct PointMaterialKey {
    r_bits: u32,
    g_bits: u32,
    b_bits: u32,
    a_bits: u32,
    luminance_bits: u32,
}

impl PointMaterialKey {
    pub fn from_color_and_luminance(color: Color, luminance: f32) -> Self {
        let srgba = color.to_srgba();
        Self {
            r_bits: srgba.red.to_bits(),
            g_bits: srgba.green.to_bits(),
            b_bits: srgba.blue.to_bits(),
            a_bits: srgba.alpha.to_bits(),
            luminance_bits: luminance.to_bits(),
        }
    }

    pub fn get_color(&self) -> Color {
        Color::srgba(
            f32::from_bits(self.r_bits),
            f32::from_bits(self.g_bits),
            f32::from_bits(self.b_bits),
            f32::from_bits(self.a_bits),
        )
    }

    pub fn get_luminance(&self) -> f32 {
        f32::from_bits(self.luminance_bits)
    }
}
