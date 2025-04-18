use crate::events::star_clicked::StarClickedEvent;
use crate::plugins::star_events::star_clicked::handle_star_clicked;
use bevy::prelude::{App, Plugin, Update};

mod star_clicked;

pub struct StarEventsPlugin;

impl Plugin for StarEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_star_clicked)
            .add_event::<StarClickedEvent>();
    }
}
