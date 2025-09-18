use std::fs;
use std::io::Write;
use dirs::config_dir;
use std::path::{PathBuf};
use once_cell::sync::Lazy;


mod help_msg;
mod config;

static DATA_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| get_path("data"));
static ARCHIVE_FILE_PATH: Lazy<PathBuf> = Lazy::new(|| get_path("archive"));

fn get_path(file: &str) -> PathBuf {
    config_dir()
        .unwrap()
        .join(format!("tosk/{}.txt", file))
}

pub fn help() {
    help_msg::text();
}

pub fn list() {
    match fs::read_to_string(&*DATA_FILE_PATH) {
        Ok(contents) => list_cont(contents),
        Err(_) => create_file(&*DATA_FILE_PATH, "list"),
    }
}

fn list_cont(contents: String) {
    if contents == "" {
        println!("The task list is empty. To add a task: \"tosk add [TASK]\"");
    }
    for (index, line) in contents.lines().rev().enumerate() {
        println!("{}. {}", index + 1, line);
    }
}

fn create_file(path: &PathBuf, origin: &str) {
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
    
    fs::File::create(path).expect("Cannot create file");
}

pub fn add(task: String) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&*DATA_FILE_PATH)
        .expect("Cannot open file");

    writeln!(file, "{}", task).expect("Cannot write to file");
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