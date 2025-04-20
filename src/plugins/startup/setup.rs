use crate::bundles::custom_camera::CustomCamera;
use crate::events::star_cluster_generate::StarClusterGenerateEvent;
use crate::resources::selected_star::SelectedStar;
use crate::resources::star_store::StarStore;
use crate::types::cluster_generation_method::ClusterGenerationMethod;
use bevy::prelude::{ClearColor, Color, Commands, EventWriter, ResMut};

pub fn setup(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut star_cluster_generate_event: EventWriter<StarClusterGenerateEvent>,
) {
    commands.spawn(CustomCamera::default());
    clear_color.0 = Color::BLACK;

    commands.insert_resource(StarStore::default());
    commands.insert_resource(SelectedStar::default());

    let method = ClusterGenerationMethod::cubic_with_seed(69);
    star_cluster_generate_event.send(StarClusterGenerateEvent::new(method));
}
