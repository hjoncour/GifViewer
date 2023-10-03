/* IMPORTS */

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

/* MENU */

pub fn create_app_menu() -> Menu {
    let file_menu: Submenu = Submenu::new("File", Menu::new()
                                .add_item(CustomMenuItem::new("new".to_string(), "New").accelerator("CmdOrCtrl+N"))
                                .add_item(CustomMenuItem::new("open".to_string(), "Open").accelerator("CmdOrCtrl+O"))
                                .add_item(CustomMenuItem::new("print".to_string(), "Print").accelerator("CmdOrCtrl+P"))
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
