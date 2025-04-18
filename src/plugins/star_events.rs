use crate::events::star_clicked::StarClickedEvent;
use crate::events::star_unselect::StarUnselectEvent;
use crate::plugins::star_events::star_clicked::handle_star_clicked;
use crate::plugins::star_events::star_unselect::handle_star_unselect;
use bevy::prelude::{App, IntoSystemConfigs, Plugin, Update};

mod star_clicked;
mod star_unselect;

pub struct StarEventsPlugin;

impl Plugin for StarEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_star_clicked, handle_star_unselect))
            .add_event::<StarClickedEvent>()
            .add_event::<StarUnselectEvent>();
    }
}
