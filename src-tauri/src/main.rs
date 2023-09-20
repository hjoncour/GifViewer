/* ATTRIBUTES */
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_substystem = "windows"
)]

/* IMPORTS */



extern crate base64;

use std::fs::File;
use std::io::{self, Read, Write};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};



// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu}; // for later

/* COMMANDS */      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn get_base64(path: String) -> String {
    let mut file: File = File::open(path).expect("Failed to open file");
    let mut file_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_data).expect("Failed to read file");
    let encoded_file: String = base64::encode(&file_data);
    return encoded_file;
}


/* MENU */

fn create_app_menu() -> Menu {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    /*let quit:       CustomMenuItem = CustomMenuItem::new("quit".to_string(), "Quit");
    let close:      CustomMenuItem = CustomMenuItem::new("close".to_string(), "Close");
    let open:       CustomMenuItem = CustomMenuItem::new("open".to_string(), "Open...");

    let submenu:    Submenu        = Submenu::new("File", Menu::new()
                                        .add_item(quit)
                                        .add_item(close)
                                        .add_item(open));
    let menu:       Menu           = Menu::new()
                                            .add_native_item(MenuItem::Copy)
                                            .add_item(CustomMenuItem::new("hide", "Hide"))
                                            .add_submenu(submenu);
    return menu;
    */
    let file_menu: Submenu = Submenu::new("File", Menu::new()
                                .add_item(CustomMenuItem::new("new".to_string(), "New").accelerator("CmdOrCtrl+N"))
                                .add_item(CustomMenuItem::new("open".to_string(), "Open").accelerator("CmdOrCtrl+O"))
                                .add_item(CustomMenuItem::new("save".to_string(), "Save").accelerator("CmdOrCtrl+S")));
    
    let view_menu: Submenu = Submenu::new("View", Menu::new()
                                .add_item(CustomMenuItem::new("previous".to_string(), "Previous").accelerator("CmdOrCtrl+ArrowLeft"))
                                .add_item(CustomMenuItem::new("next".to_string(), "Next").accelerator("CmdOrCtrl+ArrowRight"))
                                .add_item(CustomMenuItem::new("first".to_string(), "First").accelerator("CmdOrCtrl+ArrowUp"))
                                .add_item(CustomMenuItem::new("last".to_string(), "Last").accelerator("CmdOrCtrl+ArrowDown"))

                            );

    let main_menu: Submenu = Submenu::new("App", Menu::new()
                                .add_native_item(MenuItem::Quit));

                        

    let menu: Menu = Menu::new()
                                .add_submenu(main_menu)
                                .add_submenu(file_menu)
                                .add_submenu(view_menu);

    return menu;

}



/* MAIN */

fn main() {

    tauri::Builder::default()
        .menu(create_app_menu())
        .on_menu_event(|event: tauri::WindowMenuEvent|
        match event.menu_item_id() {
            "new"       =>      println!("Placeholder for new"),
            "open"      =>      println!("Open menu item clicked"),
            "save"      =>      println!("Save selected item"),
            "previous"  =>      println!("Previous item"),
            "next"      =>      println!("Last selected item"),
            _           =>      println!("Other")

        })
        .invoke_handler(tauri::generate_handler![get_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
