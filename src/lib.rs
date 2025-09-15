use std::fs;
use std::io::Write;

pub fn help() {
    println!("help")
}

pub fn list() {
    let contents = fs::read_to_string("data.txt")
        .expect("No \"data.txt\" file. Try \"tosk add [task]\" to create said file.");

    for (index, line) in contents.lines().rev().enumerate() {
        println!("{}. {}", index + 1, line);
    }
}

pub fn add(task: String) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("data.txt")
        .expect("cannot open file");

    writeln!(file, "{}", task).expect("cannot write to file");
}

pub fn remove(task: i32) {
    println!("task to remove: {}", task)
}