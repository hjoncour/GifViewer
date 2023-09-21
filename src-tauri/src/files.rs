
use std::{fs, collections::HashMap};

use crate::Multimedia;

pub fn list_files(dir: &std::path::Path, extensions: Vec<&str>) -> Vec<Multimedia> {
    let mut files: Vec<Multimedia> = vec![];
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path: std::path::PathBuf = entry.path();
                if path.is_file() {
                    if let Some(multimedia) = get_multimedia_info(&path, &extensions) {
                        files.push(multimedia);
                    }
                }
            }
        }
    }
    return files;
}

fn get_multimedia_info(file: &std::path::PathBuf, types: &[&str]) -> Option<Multimedia> {
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
                    //println!("{:#?}", metadata);
                    let multimedia: Multimedia = Multimedia {
                        name: substring.to_string(),
                        description: String::from("placeholder"),
                        author: String::from("placeholder"),
                        format: format.to_string(),
                        dimensions: (0, 0),
                        size_bytes: metadata.len(),
                        metadata: HashMap::new(),
                        content: Vec::new(),
                    };
                    return Some(multimedia);
                }
            }
        }
    }
    None
}


