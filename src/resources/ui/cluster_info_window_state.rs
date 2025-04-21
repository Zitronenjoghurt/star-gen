use crate::types::sort_direction::SortDirection;
use bevy::prelude::Resource;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ClusterInfoSortType {
    #[default]
    None,
    Mass,
    Radius,
    Luminosity,
    SurfaceTemperature,
}

#[derive(Debug, Default, Resource)]
pub struct ClusterInfoWindowState {
    pub sort_direction: SortDirection,
    pub sort_type: ClusterInfoSortType,
}

impl ClusterInfoWindowState {
    pub fn on_type_clicked(&mut self, sort_type: ClusterInfoSortType) {
        if sort_type == self.sort_type {
            match self.sort_direction {
                SortDirection::Ascending => self.sort_direction = SortDirection::Descending,
                SortDirection::Descending => self.sort_type = ClusterInfoSortType::None,
            }
        } else {
            self.sort_type = sort_type;
            self.sort_direction = SortDirection::Ascending;
        }
    }
}
