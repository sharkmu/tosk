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
    

    let mut list: Vec<DataJson> = if Path::new(&*DATA_FILE_PATH).exists() {
        let text = fs::read_to_string(&*DATA_FILE_PATH)
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
    fs::write(&*DATA_FILE_PATH, json)
        .expect("Unable to write JSON file");
}

pub fn remove(task: i32) {
    match fs::read_to_string(&*DATA_FILE_PATH) {
        Ok(contents) => rm_cont(contents, task),
        Err(_) => create_file(&*DATA_FILE_PATH, "rm"),
    }
}

fn rm_cont(contents: String, task: i32) {
    let mut lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    let lines_length: i32 = lines.len().try_into().expect("value error");
    let index_to_remove = ((task - lines_length) * (-1)) as usize;

    if index_to_remove >= lines.len() {
        eprintln!("No such task with that index number!");
    } else {
        let removed = lines.remove(index_to_remove);
        if config::load() {
            archive_removed(removed);
        }
    }

    let mut file = fs::File::create(&*DATA_FILE_PATH).expect("Cannot open file");
    for line in lines {
        writeln!(file, "{}", line).expect("Cannot write to file");
    }
}

fn archive_removed(removed: String) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&*ARCHIVE_FILE_PATH)
        .expect("Cannot open file");

    writeln!(file, "{}", removed).expect("Cannot write to file");
}