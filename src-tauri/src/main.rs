/* ATTRIBUTES */

/* IMPORTS */
extern crate base64;

use std::env; 
use std::fs::File;
use std::io::{self, Read, Write};

// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu}; // for later

/* COMMANDS */      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn get_base64(path: String) -> String {
    let mut file: File = File::open(path).expect("Failed to open file");
    let mut file_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_data).expect("Failed to read file");
    let encoded_file: String = base64::encode(&file_data);

    // _create_test(&encoded_file);

    return encoded_file;
}

fn _create_test(base64_string: &String) -> io::Result<()> {
    println!("creating test file");
    let file_path: &str = "/Users/----/Desktop/GifViewer/test/test.gif"; // Replace with your absolute file path

    let mut file: File = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return Err(e);
        }
    };
    let decoded_file: Vec<u8> = base64::decode(base64_string).unwrap();
    match file.write_all(&decoded_file) {
        Ok(()) => {
            println!("Data written successfully.");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error writing to file: {}", e);
            Err(e)
        }
    }
}

/* MAIN */

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_base64])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
