use core::panic;
use std::fs;
use std::io::Write;
use dirs::config_dir;
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use chrono::Local;


mod help_msg;
mod config;

#[derive(Serialize, Deserialize, Debug)]
struct DataJson {
    content: String,
    creation_time: String,
}

static DATA_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| get_path("data"));
static ARCHIVE_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| get_path("archive"));

fn get_path(file: &str) -> PathBuf {
    config_dir()
        .unwrap()
        .join(format!("tosk/{}.json", file))
}

pub fn help() {
    help_msg::text();
}

pub fn list() {
    match fs::read_to_string(&*DATA_FILE_PATH) {
        Ok(text) => list_cont(text),
        Err(_) => create_file(&*&DATA_FILE_PATH, "list"),
    };
}

fn list_cont(contents: String) {
    let json: Vec<DataJson> = serde_json::from_str(&contents).expect("JSON was not well-formatted");
    
    if json.len() < 1 {
        println!("The task list is empty. To add a task: \"tosk add [TASK]\"");
    } else {
        let mut index = 1;
        for entry in json.iter().rev() {
            println!("{}. {}", index, entry.content);
            index += 1;
        }
    }
}

fn create_file(path: &PathBuf, origin: &str) {
    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Failed to create parent directories for {:?}: {}", path, e);
            return;
        }
    }

    match origin {
        "list" => {
            println!("The task list is empty. To add a task: \"tosk add [TASK]\"")
        }
        "rm" => {
            println!("No such task with that index number!")
        }
        _ => {
            println!("No such origin: {}", origin)
        }
    }

    let mut file = fs::File::create(&path).expect("Cannot write to file.");
    write!(file, "[]").expect("Cannot write to file");
}

pub fn add(content: String) {
    let entry = DataJson {
        content: content,
        creation_time: Local::now().to_rfc3339(),
    };
    
    write_to_json(entry, "data");
}

pub fn remove(task: i32) {
    let contents = 
        fs::read_to_string(&*DATA_FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    let mut json: Vec<DataJson> = 
        serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());

    if json.len() < 1 {
        create_file(&*&DATA_FILE_PATH, "rm");
    } else {
        let json_length: i32 = json.len().try_into().unwrap();
        if task > 0 && task < json_length+1{
            let index_to_remove = task as usize;
            let removed = json.remove(json.len() - index_to_remove);

            let new_json = serde_json::to_string_pretty(&json)
                .expect("Failed to serialize JSON");
            fs::write(&*DATA_FILE_PATH, new_json)
                .expect("Unable to write JSON file");

            if config::load() {
                archive_removed(removed);
            }
        } else {
            println!("No such task with that index number!")
        }
        
    }
}

fn archive_removed(removed: DataJson) {
    let entry = DataJson {
        content: removed.content,
        creation_time: removed.creation_time,
    };
    write_to_json(entry, "archive");
}

fn write_to_json(entry: DataJson, file: &str) {
    let file_path = match file {
        "data" => &DATA_FILE_PATH,
        "archive" => &ARCHIVE_FILE_PATH,
        _ => panic!("Invalid match option!"),
    };

    let mut list: Vec<DataJson> = if Path::new(file_path.as_path()).exists() {
        let text = fs::read_to_string(file_path.as_path())
            .expect("Unable to read JSON file");

        if text.trim().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&text)
                .expect("JSON was not well-formatted")
        }
    } else {
        Vec::new()
    };

    list.push(entry);

    let json = serde_json::to_string_pretty(&list)
        .expect("Failed to serialize JSON");
    fs::write(file_path.as_path(), json)
        .expect("Unable to write JSON file");
}