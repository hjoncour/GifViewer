/* ATTRIBUTES */
#![feature(lazy_cell)]

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_substystem = "windows"
)]

/* LOCAL */

mod formats;                                    // Types of files: jpeg, png, gif
mod menu;                                       // Top left options, navigation, save, open
mod multimedia;                                 // File informations: name, content, date, dimensions
mod files;

use formats::all_file_formats;
use multimedia::Multimedia;

/* IMPORTS */

extern crate base64;
use std::path::PathBuf;
use std::sync::LazyLock;
use serde_json::json;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, env};
use std::io::{Read};

/* STATIC */
static GLOBAL: LazyLock<Arc<Mutex<Vec<Multimedia>>>> = LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));
static LOCAL: LazyLock<Arc<Mutex<Vec<&'static Multimedia>>>> = LazyLock::new(|| Arc::new(Mutex::new(Vec::new())));
static CURRENT_PATH: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| Mutex::new(PathBuf::new()));
static ALL_PATHS: LazyLock<Mutex<HashMap<PathBuf, Vec<&'static Multimedia>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/* COMMANDS */

#[tauri::command]
fn get_base64(path: String) -> String {
    files::encode_file(path)
}

#[tauri::command]
fn next(path: String, index: usize) -> serde_json::Value {
    println!("\nnext called");
    println!("received: {}", index);
    let index_val: usize;
    let name: &String;
    let media: &String;

    let local_files: std::sync::MutexGuard<'_,  Vec<&Multimedia>> = LOCAL.lock().unwrap();

    if local_files.len() == 0 {
        sync(path);
    }
    if index == 0 || index >= local_files.len() {
        index_val   = 0;
        name        = &local_files[0].name;
        media       = &local_files[0].content;
    } else {
        index_val   = index;
        name        = &local_files[index_val].name;
        media       = &local_files[index_val].content;
    }
    return json!({"index": index_val, "name": name, "media": media});
}

#[tauri::command]
fn previous(path: String, index: usize) -> serde_json::Value {
    println!("\nprevious called");
    println!("received: {}", index);
    let index_val: usize;
    let name: &String;
    let media: &String;
    let local_files: std::sync::MutexGuard<'_,  Vec<&Multimedia>> = LOCAL.lock().unwrap();
    if local_files.len() == 0 {
        sync(path);
    }
    if index >= local_files.len() {
        index_val   = 0;
        name        = &local_files[0].name;
        media       = &local_files[0].content;
    } else {
        index_val   = index;
        name        = &local_files[index_val].name;
        media       = &local_files[index_val].content;
    }
    return json!({"index": index_val, "name": name, "media": media});
}

#[tauri::command]
fn save(index: usize) -> serde_json::Value {
    let local_files: std::sync::MutexGuard<'_, Vec<&Multimedia>> = LOCAL.lock().unwrap();

    if index >= local_files.len() {
        return serde_json::json!({ "error": "Invalid index" });
    }

    let target: &Multimedia = &local_files[index];
    let content_base64: &String = &target.content;
    let content_bytes: Vec<u8> = base64::decode(content_base64).unwrap();
    let file_name: String = files::get_new_file_name(&target.name, &target.format);

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
fn sync(path: String) {
    let status: &str;
    let message: &str;
    println!("In sync");
    println!("Current path: {:?}", get_current_path());
    let current_dir: PathBuf = name_path_to_path(path);
    let mut all_paths = ALL_PATHS.lock().unwrap();
    if current_dir != get_current_path() {
        update_current_path(current_dir.clone());                                        
    }                                                                                           
    let all_paths_entry: std::collections::hash_map::Entry<'_, PathBuf, Vec<&Multimedia>> = all_paths.entry(current_dir.clone());

    if all_paths_entry.or_insert_with(Vec::new).is_empty() {
        let current_dir_clone = current_dir.clone();
        thread::spawn(move || {
            let files: Vec<Multimedia> = files::list_files(&current_dir_clone, all_file_formats());

            let mut global_files: std::sync::MutexGuard<'_, Vec<Multimedia>> = GLOBAL.lock().unwrap();
            global_files.extend(files.clone());

            let mut local_references: std::sync::MutexGuard<'_, Vec<&'static Multimedia>> = LOCAL.lock().unwrap();
            for global_file in &*global_files {
                local_references.push(&*Box::leak(Box::new(global_file.clone())));
            }
        });
    }
    return;
}



/* PATH */
fn name_path_to_path(name_path: String) -> PathBuf {
    let path = PathBuf::from(name_path);
    path.parent().unwrap_or(&path).to_path_buf()
}

fn get_current_path() -> PathBuf {
    let current_path: std::sync::MutexGuard<'_, PathBuf> = CURRENT_PATH.lock().unwrap();
    current_path.clone()
}

fn update_current_path(new_path: PathBuf) {
    let mut current_path = CURRENT_PATH.lock().unwrap();
    *current_path = new_path;
}

/* MAIN */
fn main() {
    /* BUILD APP  */
    tauri::Builder::default()
        .menu(menu::create_app_menu())
        .on_menu_event(|event: tauri::WindowMenuEvent|
        match event.menu_item_id() {
            "new"       =>      println!("Placeholder for new"),
            "open"      =>      event.window().emit("open-file", "open").unwrap(),
            "save"      =>      event.window().emit("save-file", "save").unwrap(),
            "print"     =>      event.window().emit("print-file", "print").unwrap(),
            "previous"  =>      event.window().emit("previous-item", "previous").unwrap(),
            "next"      =>      event.window().emit("next-item", "next").unwrap(),
            "first"     =>      event.window().emit("first-item", "first").unwrap(),
            "last"      =>      event.window().emit("last-item", "last").unwrap(),
            _           =>      println!("Other")
        })
        .invoke_handler(tauri::generate_handler![get_base64, next, previous, save, sync])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
