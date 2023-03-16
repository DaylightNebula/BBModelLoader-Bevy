use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::tracing::Instrument;
use bevy::winit::WinitSettings;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_event::<AppExit>()
        .add_system(global_init.in_schedule(OnEnter(AppState::Running)))
        .add_system(global_update.in_set(OnUpdate(AppState::Running)))
        .insert_resource(WinitSettings { return_from_run: true, ..default() })
        .run();

    global_exit();
}

fn global_init() {
}

fn global_update() {
}

fn global_exit() {
}

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    Running
}