use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct SelectedPoint {
    id: Option<u64>,
}
