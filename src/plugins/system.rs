use crate::plugins::system::load_on_startup::load_on_startup;
use crate::plugins::system::save_on_exit::save_on_exit;
use crate::plugins::system::setup::setup;
use bevy::app::App;
use bevy::prelude::{on_event, AppExit, IntoSystemConfigs, Last, Plugin, PostStartup, Startup};

mod load_on_startup;
mod save_on_exit;
mod setup;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(PostStartup, load_on_startup);
        app.add_systems(Last, save_on_exit.run_if(on_event::<AppExit>));
    }
}
