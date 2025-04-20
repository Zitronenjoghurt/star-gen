use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct WindowManager {
    pub bloom_settings: bool,
    pub cluster_info: bool,
    pub diagnostics: bool,
    pub graphics_settings: bool,
    pub settings: bool,
    pub generate_cubic_modal: bool,
    pub generate_seeded_modal: bool,
}
