use crate::types::cluster_generation_method::ClusterGenerationMethod;
use bevy::prelude::Event;

#[derive(Debug, Default, Event)]
pub struct StarClusterGenerateEvent(ClusterGenerationMethod);

impl StarClusterGenerateEvent {
    pub fn new(method: ClusterGenerationMethod) -> Self {
        Self(method)
    }

    pub fn get_method(&self) -> &ClusterGenerationMethod {
        &self.0
    }
}
