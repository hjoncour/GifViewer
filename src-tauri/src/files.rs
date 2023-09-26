
use std::{fs::{self, File}, collections::HashMap, io::{Read, Write}};
extern crate base64;
use crate::Multimedia;

pub fn list_files(dir: &std::path::Path, extensions: Vec<&str>) -> Vec<Multimedia> {
    let mut files: Vec<Multimedia> = vec![];
    if let Ok(entries) = fs::read_dir(dir) {
        let mut local_index: usize = 0;
        for entry in entries {
            if let Ok(entry) = entry {
                let path: std::path::PathBuf = entry.path();
                if path.is_file() {
                    if let Some(multimedia) = get_multimedia_info(&path, &extensions, local_index) {
                        files.push(multimedia);
                        local_index += 1;
                    }
                }
            }
        }
    }
    return files;
}

fn get_multimedia_info(file: &std::path::PathBuf, types: &[&str], local_index: usize) -> Option<Multimedia> {
    let path: &&std::path::PathBuf = &file;
    let input: Option<&str> = file.to_str();
    let input_str: &str = match input {
        Some(s) => s,
        None => return None,
    };
    
    for format in types {
        if let Some(last_slash_index) = input_str.rfind('/') {
            let substring: &str = &input_str[last_slash_index + 1..];
            if substring.contains(format) {
                if let Ok(metadata) = fs::metadata(file) {
                    let content = encode_file(file.clone().into_os_string().into_string().unwrap());                   
                        let multimedia: Multimedia = Multimedia {
                        name:           substring.to_string(),
                        local_index:    local_index,
                        description:    String::from("placeholder"),
                        author:         String::from("placeholder"),
                        format:         format.to_string(),
                        file_type:      String::from("type"),
                        dimensions:     (0, 0),
                        size_bytes:     metadata.len(),
                        metadata:       HashMap::new(),
                        content:        content,
                    };
                    return Some(multimedia);
                }
            }
        }
    }
    None
}

pub fn encode_file(path: String) -> String {
    let mut file: File = File::open(path).expect("Failed to open file");
    let mut file_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_data).expect("Failed to read file");
    let encoded_file: String = base64::encode(&file_data);
    return encoded_file;
}
