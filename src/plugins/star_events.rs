use crate::events::star_clicked::StarClickedEvent;
use crate::events::star_cluster_generate::StarClusterGenerateEvent;
use crate::events::star_delete::StarDeleteEvent;
use crate::events::star_delete_all::StarDeleteAllEvent;
use crate::events::star_spawn::StarSpawnEvent;
use crate::events::star_unselect::StarUnselectEvent;
use crate::plugins::star_events::star_clicked::handle_star_clicked;
use crate::plugins::star_events::star_cluster_generate::handle_star_cluster_generate;
use crate::plugins::star_events::star_delete::handle_star_delete;
use crate::plugins::star_events::star_delete_all::handle_star_delete_all;
use crate::plugins::star_events::star_spawn::handle_star_spawn;
use crate::plugins::star_events::star_unselect::handle_star_unselect;
use bevy::prelude::{on_event, App, IntoSystemConfigs, Plugin, Update};

mod star_clicked;
mod star_cluster_generate;
mod star_delete;
mod star_delete_all;
mod star_spawn;
mod star_unselect;

pub struct StarEventsPlugin;

impl Plugin for StarEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    handle_star_cluster_generate.run_if(on_event::<StarClusterGenerateEvent>),
                    handle_star_delete_all.run_if(on_event::<StarDeleteAllEvent>),
                    handle_star_spawn.run_if(on_event::<StarSpawnEvent>),
                )
                    .chain(),
                handle_star_clicked.run_if(on_event::<StarClickedEvent>),
                handle_star_delete.run_if(on_event::<StarDeleteEvent>),
                handle_star_unselect.run_if(on_event::<StarUnselectEvent>),
            ),
        )
        .add_event::<StarClickedEvent>()
        .add_event::<StarDeleteEvent>()
        .add_event::<StarDeleteAllEvent>()
        .add_event::<StarClusterGenerateEvent>()
        .add_event::<StarSpawnEvent>()
        .add_event::<StarUnselectEvent>();
    }
}
