extern crate reqwest;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread;
use bevy::app::{App, Plugin};
use bevy::prelude::Commands;
use reqwest::Client;
use tokio::runtime::*;
use tokio::task::JoinHandle;

pub fn download_file_async(internal_path: String) {
    thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let response = reqwest::get(&internal_path).await.expect("Could not download file");

            println!("Received response, getting path...");
            let path = Path::new("./download.txt");

            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}", why),
                Ok(file) => file,
            };
            let content = response.bytes().await.expect("ERROR");
            file.write_all(content.as_ref()).expect("ERROR");
            return file
        });
    });
}