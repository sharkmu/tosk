use std::{fs, io};
use std::io::Write;
use dirs::config_dir;
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use chrono::Local;


mod help_msg;
mod config;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataJson {
    content: String,
    creation_time: String,
}

static DATA_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| create_path("data"));
static ARCHIVE_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| create_path("archive"));

fn create_path(file: &str) -> PathBuf {
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
        Err(_) => create_file(&*DATA_FILE_PATH, "list"),
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

pub fn info(task: i32) {
    let contents = fs::read_to_string(&*DATA_FILE_PATH)
        .expect("Unable to read JSON file");
    let json: Vec<DataJson> = serde_json::from_str(&contents)
        .expect("JSON was not well-formatted");

    let json_length: i32 = json.len().try_into().unwrap();
    if task > 0 && task < json_length+1 {
        let index = (json_length - task) as usize;
        
        let entry = &json[index];
        let number_suffix = match task {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!("Here is the information I found on the {}{} entry of the task list:", task, number_suffix);
        println!("  - content: {}", entry.content);
        println!("  - creation time: {}", entry.creation_time);
    } else {
        println!("No such task with that index number!");
    }
}

pub fn edit(task: i32) {
    let mut json: Vec<DataJson> = read_json();

    let json_length: i32 = json.len().try_into().unwrap();
    if task > 0 && task < json_length+1 {
        let index = (json_length - task) as usize;
        
        let entry: &mut DataJson = &mut json[index];
        let number_suffix = match task {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!("Here is the old content of the {}{} entry of the task list:", task, number_suffix);
        print!("{}\nType in the new content: ", entry.content);
        io::stdout().flush().unwrap();
        // without flushing the input is taken first, then the prompt is printed

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                entry.content = input.trim().to_string();
                write_list_to_json(json, &*DATA_FILE_PATH);
            }
            Err(_) => {
                println!("Error on reading input!");
            }
        }
    } else {
        println!("No such task with that index number!");
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
    let current_local = Local::now();
    let formatted_local_time = current_local.format("%Y-%m-%d %H:%M:%S");
    let entries: Vec<DataJson> = content.split("&").map(|task|{
        DataJson {
            content: task.to_string(),
            creation_time: formatted_local_time.to_string(),
        }
    }).collect();

    write_list_to_json(entries, &*DATA_FILE_PATH);
}

pub fn rm_all() {
    let mut sure = String::new();

    print!("Are you sure that you want to delete all tasks? (Y/n): ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut sure)
        .expect("Failed to read line");

    let sure = sure.trim();

    if sure == "Y" {
        if config::load() {
            let contents = fs::read_to_string(&*DATA_FILE_PATH);
            let json: Vec<DataJson> = serde_json::from_str(&*contents.unwrap())
                .expect("JSON was not well-formatted");
            for entry in json.iter() {
                archive_removed(entry.clone());
            }
        }

        fs::write(&*DATA_FILE_PATH, "[]")
            .expect("Unable to write JSON file");
    }
}

fn read_json() -> Vec<DataJson> {
    let contents = 
        fs::read_to_string(&*DATA_FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    let json: Vec<DataJson> = 
        serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
    json
}

pub fn reorder(task: i32, new_position: i32) {
    let mut json: Vec<DataJson> = read_json();
    match (
        task > (json.len()) as i32 || task<=0,
        new_position > (json.len()) as i32 || new_position <= 0
    ) {
        (true, true) => {
            println!("Wrong task number! It must be between 0 and {}", json.len());
            println!("Wrong position! It must be between 0 and {}", json.len());
        }
        (true, false) => {
            println!("Wrong task number! It must be between 0 and {}", json.len());
        }
        (false, true) => {
            println!("Wrong position! It must be between 0 and {}", json.len());
        }
        (false, false) => {
            let corrected_task = json.len()-1-((task as usize)-1);
            let corrected_new = json.len()-1-((new_position as usize)-1);
            let removed = json.remove(corrected_task);
            json.insert(corrected_new, removed);
            write_list_to_json(json, &*DATA_FILE_PATH);
        }
    };
}

pub fn remove(task: i32) {
    let mut json = read_json();

    if json.len() < 1 {
        create_file(&*DATA_FILE_PATH, "rm");
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
    write_to_json(entry, &*ARCHIVE_FILE_PATH);
}

fn write_list_to_json(list: Vec<DataJson>, file_path: &PathBuf) {
    let json = serde_json::to_string_pretty(&list)
        .expect("Failed to serialize JSON");
    fs::write(file_path, json)
        .expect("Unable to write JSON file");
}

fn get_list(file_path: &PathBuf) -> Vec<DataJson>{
    if Path::new(file_path.as_path()).exists() {
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
    }
}

fn write_to_json(entry: DataJson, file: &PathBuf) {
    let mut list = get_list(file);
    list.push(entry);
    write_list_to_json(list, &file);
}
