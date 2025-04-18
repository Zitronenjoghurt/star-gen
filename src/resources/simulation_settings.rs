use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct SimulationSettings {
    pub wireframe: bool,
    pub bloom: bool,
}
