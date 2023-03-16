use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::tracing::Instrument;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        // .add_plugin(Initializer)
        .add_event::<AppExit>()
        .add_system(global_update)
        .add_startup_system(global_init)
        .add_system(global_exit)
        .run();
}

fn global_init() { }

fn global_update() {
}

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    Init,
    Running
}