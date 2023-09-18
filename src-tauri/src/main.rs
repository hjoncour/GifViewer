/* ATTRIBUTES */

/* IMPORTS */

use std::env; 
use std::fs::File;
use std::io::{self, Read, Write};
use base64::{URL_SAFE, encode_config, decode};

// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu}; // for later

/* COMMANDS */      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn get_base64(path: String) -> String {
    println!("get_base64(path: String)");
    println!("---");
    println!("{}", path);

    let mut file: File = File::open(path).expect("Failed to open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let x: String = encode_config(&buffer, URL_SAFE);
    create_test(&x);
    println!("{}: ", &x);
    return x;
}

fn create_test(base64_string: &String) -> io::Result<()> {
    println!("creating test file");
    let file_path = "/Users/hugo/Desktop/GifViewer/test/test.gif"; // Replace with your absolute file path

    // Create or open the file for writing
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return Err(e);
        }
    };

    // Your String to be written to the file
    let content_to_write: &String = base64_string;

    // Convert the String into bytes (UTF-8 encoding in this example)
    let content_bytes = content_to_write.as_bytes();

    // Write the bytes to the file
    match file.write_all(content_bytes) {
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
