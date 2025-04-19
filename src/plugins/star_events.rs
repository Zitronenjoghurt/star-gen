use crate::events::star_clicked::StarClickedEvent;
use crate::events::star_delete::StarDeleteEvent;
use crate::events::star_delete_all::StarDeleteAllEvent;
use crate::events::star_spawn::StarSpawnEvent;
use crate::events::star_unselect::StarUnselectEvent;
use crate::plugins::star_events::star_clicked::handle_star_clicked;
use crate::plugins::star_events::star_delete::handle_star_delete;
use crate::plugins::star_events::star_delete_all::handle_star_delete_all;
use crate::plugins::star_events::star_spawn::handle_star_spawn;
use crate::plugins::star_events::star_unselect::handle_star_unselect;
use bevy::prelude::{App, Plugin, Update};

mod star_clicked;
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
                handle_star_clicked,
                handle_star_delete,
                handle_star_delete_all,
                handle_star_spawn,
                handle_star_unselect,
            ),
        )
        .add_event::<StarClickedEvent>()
        .add_event::<StarDeleteEvent>()
        .add_event::<StarDeleteAllEvent>()
        .add_event::<StarSpawnEvent>()
        .add_event::<StarUnselectEvent>();
    }
}
