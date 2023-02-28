/* ATTRIBUTES */

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/* IMPORTS */

use std::env; 
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};



/* COMMANDS */      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn path() -> String {
    println!("REQUESTED PATH");
    return format!("path: {:?}", env::current_dir());
}

#[tauri::command]
fn default_image() -> String {
    println!("REQUEST DEFAULT IMAGE"); // direct path
    return format!("/Users/hugo/Desktop/GifViewer/src/assets/images/gif/fire.gif");
}

#[tauri::command]
fn relative_path_image() -> String {
    println!("REQUEST RELATIVE PATH TO IMAGE"); // relative path
    let path = &env::current_dir().unwrap().as_path().display().to_string();
    let new_path = format_path(path.to_string());
    let result = format!("{}src/assets/images/gif/not_default.gif", new_path);
    println!("{}", new_path);
    return result;
}

/* FUNCTIONS */

fn format_path(path: String) -> String {
    let index: usize = path.find("src-tauri").unwrap();
    let new_path = &path[..index];
    return new_path.to_string();
}


/* MAIN */

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![path, relative_path_image, default_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
