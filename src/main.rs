mod file_system;

use std::fs::File;
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

fn file_downloaded(file: File) {
    println!("Downloaded file {:?}", file);
}

fn global_init(commands: Commands) {
    download_file("main.scene".to_string(), FileSystemQueue::Low, FileSystemProcessor { callback: file_downloaded });
    download_file("test.scene".to_string(), FileSystemQueue::Low, FileSystemProcessor { callback: file_downloaded });
    download_file("test_menu.eml".to_string(), FileSystemQueue::High, FileSystemProcessor { callback: file_downloaded });
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