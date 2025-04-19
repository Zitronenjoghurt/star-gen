use crate::resources::settings::wireframe::WireframeSettings;
use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::{Res, ResMut};

pub fn apply_wireframe_settings(
    wireframe_settings: Res<WireframeSettings>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = wireframe_settings.active;
}
