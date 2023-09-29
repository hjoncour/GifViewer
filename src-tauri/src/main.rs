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
    println!("\nnext called");
    println!("received: {}", index);
    
    let local_files: std::sync::MutexGuard<'_, Vec<Multimedia>> = LOCAL.lock().unwrap();

    for file in &*local_files {
        println!("{}: {}", file.local_index, file.name);
    }

    println!("index: {}\t list: {}\t index >= list:{}", index, local_files.len(), index >= local_files.len());
    if index <= 0 || index >= local_files.len() {
        println!("case 1");
        let data: serde_json::Value = json!({
            "index": 0,
            "name": &local_files[0].name,
            "media": &local_files[0].content,
        });
        println!("sent index: {}, {}", index, local_files[index].name);
        return data;
    } else {
        println!("case 2");
        let data: serde_json::Value = json!({
            "index": index,
            "name": &local_files[0].name,
            "media": &local_files[index].content,
        });
        println!("sent index: {}, {}", index, local_files[index].name);
        return data;
    }
}

#[tauri::command]
fn save(index: usize) -> serde_json::Value {
    println!("\nsave called");

    let local_files: std::sync::MutexGuard<'_, Vec<Multimedia>> = LOCAL.lock().unwrap(); // TO FIX

    if index < 0 || index >= local_files.len() {
        return serde_json::json!({ "error": "Invalid index" });
    }

    let target: &Multimedia = &local_files[index];
    let content_base64: &String = &target.content;
    let content_bytes: Vec<u8> = base64::decode(content_base64).unwrap();

    let file_name: String = files::get_new_file_name(&target.name, &target.format);

    // Create a new file and write the content to it.
    match std::fs::write(&file_name, &content_bytes) {
        Ok(_) => {
            println!("File '{}' successfully saved.", &file_name);
            serde_json::json!({ "message": format!("File '{}' saved.", &file_name) })
        }
        Err(err) => {
            eprintln!("Error saving file '{}': {:?}", &file_name, err);
            serde_json::json!({ "error": format!("Error saving file '{}'", &file_name) })
        }
    }
}


#[tauri::command]
fn sync(path: String) -> serde_json::Value {
    let status: &str;
    let message: &str;
    if true {
        status = "success";
        message = "Operation succeeded";
    } else {
        status = "failure";
        message = "Operation failed";
    }
    println!("status");
    return serde_json::json!({"status": status, "message": message});
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
        .invoke_handler(tauri::generate_handler![get_base64, next, save, sync])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
