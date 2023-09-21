/* ATTRIBUTES */

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_substystem = "windows"
)]

/* IMPORTS */

extern crate base64;
mod formats;
mod menu;
mod multimedia;
mod files;

use multimedia::Multimedia;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{self, Read, Write};
use formats::all_file_formats;

/* COMMANDS */

#[tauri::command]
fn get_base64(path: String) -> String {
    let mut file: File = File::open(path).expect("Failed to open file");
    let mut file_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_data).expect("Failed to read file");
    let encoded_file: String = base64::encode(&file_data);
    return encoded_file;
}

/* MAIN */

fn main() {

    let current_dir: std::path::PathBuf = std::env::current_dir().expect("Failed to get current directory");
    let result: Arc<Mutex<Vec<Multimedia>>> = Arc::new(Mutex::new(Vec::new()));
    let result_clone: Arc<Mutex<Vec<Multimedia>>> = Arc::clone(&result);

    let handle: thread::JoinHandle<()> = thread::spawn(move || {
        let files: Vec<Multimedia> = files::list_files(&current_dir, all_file_formats());
        let mut result: std::sync::MutexGuard<'_, Vec<Multimedia>> = result_clone.lock().unwrap();
        result.extend(files);
    });
    handle.join().unwrap();

    // Access and iterate through the result vector
    let result: std::sync::MutexGuard<'_, Vec<Multimedia>> = result.lock().unwrap();
    for file in &*result {
        // Do something with each file path (display its content, for example)
        println!("Watch this struct title: {}", file.title);
    }



    tauri::Builder::default()
        .menu(menu::create_app_menu())
        .on_menu_event(|event: tauri::WindowMenuEvent|
        match event.menu_item_id() {
            "new"       =>      println!("Placeholder for new"),
            "open"      =>      event.window().emit("open-file", "open").unwrap(),              //println!("Open menu item clicked"),
            "save"      =>      event.window().emit("save-file", "save").unwrap(),              //println!("Save selected item"),
            "previous"  =>      event.window().emit("previous-item", "previous").unwrap(),      //println!("Previous item"),
            "next"      =>      event.window().emit("next-item", "next").unwrap(),              //println!("Last selected item"),
            "first"     =>      event.window().emit("first-item", "first").unwrap(),            //println!("Last selected item"),
            "last"      =>      event.window().emit("last-item", "last").unwrap(),              //println!("Last selected item"),
            _           =>      println!("Other")

        })
        .invoke_handler(tauri::generate_handler![get_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
