mod file_system;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::utils::tracing::Instrument;
use bevy::winit::WinitSettings;
use futures::executor::block_on;
use crate::file_system::*;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_event::<AppExit>()
        .add_startup_system(global_init)
        .add_system(global_update)
        .insert_resource(WinitSettings { return_from_run: true, ..default() }) // may cause problems on some platforms
        .run();

    global_exit();
}

fn global_init(commands: Commands) {
    println!("Calling download!");
    download_file_async("http://localhost:8000/main.scene".to_string());
    println!("hi");
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