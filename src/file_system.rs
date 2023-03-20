extern crate reqwest;

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path};
use std::slice::Iter;
use std::{fmt, thread};
use std::fmt::{Formatter, write};
use bevy::app::{App, Plugin};
use bevy::prelude::Commands;
use bevy::utils::HashMap;
use lazy_static::lazy_static;
use reqwest::Client;
use tokio::runtime::*;
use tokio::task::JoinHandle;
use crate::file_system::FileSystemQueue::{ExtremeHigh, High, Low, Medium};
use std::sync::Mutex;

// create file queue
lazy_static! {
    static ref FILE_QUEUE: Mutex<HashMap<FileSystemQueue, Vec<(String, FileSystemProcessor)>>> = {
        let mut m = HashMap::new();
        for queue in FileSystemQueue::iter() {
            m.insert(*queue, Vec::new());
        }
        return Mutex::new(m);
    };
}

pub fn download_file(internal_path: String, queue: FileSystemQueue, processor: FileSystemProcessor) {
    // if file exists, simply cancel
    let local_path = format!("./cache/{}", internal_path);
    let path = Path::new(local_path.as_str());
    if path.exists() {
        println!("File {} already exists, skipping", internal_path);
        processor.run(&File::create(&path).expect("Could not load a file from a path that exists!"));
        return;
    }

    // if extreme high, just start the download and cancel
    if queue == ExtremeHigh {
        FILE_QUEUE.lock().unwrap().get_mut(&queue).unwrap().push((internal_path, processor));
        download_file_async_from_queue(queue);
        return;
    }

    // check if all queues are empty
    let mut wait_download = false;
    for queue in FileSystemQueue::iter() {
        if FILE_QUEUE.lock().unwrap().get(&queue).unwrap().len() > 0 {
            wait_download = true
        }
    }

    // add to queue
    FILE_QUEUE.lock().unwrap().get_mut(&queue).unwrap().push((internal_path, processor));

    // if we dont need to wait for the download, just start it
    if !wait_download {
        download_file_async_from_queue(queue);
    }
}

fn download_file_async_from_queue(queue: FileSystemQueue) {
    // create thread to download async
    thread::spawn(move || {
        let pair = get_first_in_queue(&queue);
        let internal_path = &pair.0.clone();
        let processor = &pair.1;

        // get local path
        let local_path = format!("./cache/{}", internal_path);
        let path = Path::new(local_path.as_str());

        // get local file
        std::fs::create_dir_all(path.parent().unwrap()).expect(format!("Could not create parent directories for path {}", local_path).as_str());
        let mut file = File::create(&path).expect(format!("Could not create file from path {}", local_path).as_str());

        // reqwest needs a tokio runtime to run everything
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // load server path
            let server_path = format!("http://localhost:8000/{}", internal_path);
            let response = reqwest::get(server_path.as_str()).await.expect("Could not download file");

            // save content
            let content = response.bytes().await.expect(format!("Failed to get response content for server path {}", server_path).as_str());
            file.write_all(content.as_ref()).expect(format!("Failed to write to file {}", local_path).as_str());
            println!("Successfully downloaded file {}", internal_path);
        });

        processor.run(&file);

        // start the download of the first element in the highest queue
        for q2 in FileSystemQueue::iter() {
            if FILE_QUEUE.lock().unwrap().get(q2).unwrap().len() > 0 {
                download_file_async_from_queue(q2.clone());
                break;
            }
        }
    });
}

// simple function so that locks are short and sweet
fn get_first_in_queue(queue: &FileSystemQueue) -> (String, FileSystemProcessor) {
    return FILE_QUEUE.lock().unwrap().get_mut(queue).unwrap().remove(0);
}

// file system queue
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum FileSystemQueue {
    ExtremeHigh = 0,
    High = 1,
    Medium = 2,
    Low = 3
}
impl FileSystemQueue {
    pub fn iter() -> Iter<'static, FileSystemQueue> {
        static QUEUES: [FileSystemQueue; 4] = [ExtremeHigh, High, Medium, Low];
        return QUEUES.iter()
    }
}

// file system callback and processor
pub type FileSystemCallback = fn(file: &File);
#[derive(Clone, Copy)]
pub struct FileSystemProcessor {
    pub(crate) callback: FileSystemCallback
}
impl FileSystemProcessor {
    fn run(&self, file: &File) {
        (self.callback)(file);
    }
}