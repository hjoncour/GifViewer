/* ATTRIBUTES */
#![feature(lazy_cell)]

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_substystem = "windows"
)]

/* LOCAL */

mod formats;
mod menu;
mod multimedia;
mod files;

use formats::all_file_formats;
use multimedia::Multimedia;

/* IMPORTS */

extern crate base64;
use std::sync::LazyLock;
use serde_json::json;

use std::fs::File;
use std::sync::{Arc, Mutex};
use std::{thread, env};
use std::io::{Read};

/* STATIC */

static LOCAL: LazyLock<Arc<Mutex<Vec<Multimedia>>>> = LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));

/* COMMANDS */

#[tauri::command]
fn get_base64(path: String) -> String {
    files::encode_file(path)
}

#[tauri::command]
fn next(path: String, index: usize) -> serde_json::Value {
    println!("next called");
    let local_files: std::sync::MutexGuard<'_, Vec<Multimedia>> = LOCAL.lock().unwrap();
    if index <= 0 || index >= local_files.len() {
        let data: serde_json::Value = json!({
            "index": 0,
            "name": &local_files[0].name,
            "media": &local_files[0].content,
        });
        return data;
    } else {
        let data: serde_json::Value = json!({
            "index": index,
            "name": &local_files[0].name,
            "media": &local_files[index].content,
        });
        return data;
    }
}

/* MAIN */

fn main() {

    /* GET LOCAL FILES */
     let current_dir: std::path::PathBuf = std::env::current_dir().expect("Failed to get current directory");
     let local: Arc<Mutex<Vec<Multimedia>>> = Arc::new(Mutex::new(Vec::new()));
     let result_clone: Arc<Mutex<Vec<Multimedia>>> = Arc::clone(&local);
     
     let handle: thread::JoinHandle<()> = thread::spawn(move || {
        let files: Vec<Multimedia> = files::list_files(&current_dir, all_file_formats());
        let mut result: std::sync::MutexGuard<'_, Vec<Multimedia>> = LOCAL.lock().unwrap();
        result.extend(files);
    });

     handle.join().unwrap();
     
     let local_files: std::sync::MutexGuard<'_, Vec<Multimedia>> = local.lock().unwrap();
     for file in &*local_files {
        println!("Watch this struct title: {}", file.name);
     }

    /* BUILD APP  */
    tauri::Builder::default()
        .menu(menu::create_app_menu())
        .on_menu_event(|event: tauri::WindowMenuEvent|
        match event.menu_item_id() {
            "new"       =>      println!("Placeholder for new"),
            "open"      =>      event.window().emit("open-file", "open").unwrap(),
            "save"      =>      event.window().emit("save-file", "save").unwrap(),
            "previous"  =>      event.window().emit("previous-item", "previous").unwrap(),
            "next"      =>      event.window().emit("next-item", "next").unwrap(),
            "first"     =>      event.window().emit("first-item", "first").unwrap(),
            "last"      =>      event.window().emit("last-item", "last").unwrap(),
            _           =>      println!("Other")

        })
        .invoke_handler(tauri::generate_handler![get_base64, next])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
