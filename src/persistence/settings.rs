use crate::resources::settings::bloom::BloomSettings;
use crate::resources::settings::controls::ControlSettings;
use crate::resources::settings::graphics::GraphicsSettings;
use crate::resources::settings::wireframe::WireframeSettings;
use bevy::prelude::World;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SaveStateSettings {
    bloom: BloomSettings,
    controls: ControlSettings,
    graphics: GraphicsSettings,
    wireframe: WireframeSettings,
}

impl SaveStateSettings {
    pub fn save(world: &World) -> Self {
        Self {
            bloom: world.resource::<BloomSettings>().clone(),
            controls: world.resource::<ControlSettings>().clone(),
            graphics: world.resource::<GraphicsSettings>().clone(),
            wireframe: world.resource::<WireframeSettings>().clone(),
        }
    }

    pub fn load(&self, world: &mut World) {
        if let Some(mut bloom) = world.get_resource_mut::<BloomSettings>() {
            *bloom = self.bloom.clone();
        }
        if let Some(mut controls) = world.get_resource_mut::<ControlSettings>() {
            *controls = self.controls.clone();
        }
        if let Some(mut graphics) = world.get_resource_mut::<GraphicsSettings>() {
            *graphics = self.graphics.clone();
        }
        if let Some(mut wireframe) = world.get_resource_mut::<WireframeSettings>() {
            *wireframe = self.wireframe.clone();
        }
    }

    pub fn apply_validations(&mut self) {
        self.bloom.apply_validation();
        self.controls.apply_validations();
        self.graphics.apply_validations();
    }
}
