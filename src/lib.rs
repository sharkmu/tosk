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
        .expect("Cannot open file");

    writeln!(file, "{}", task).expect("Cannot write to file");
}

pub fn remove(task: i32) {
    let contents = fs::read_to_string("data.txt")
        .expect("No \"data.txt\" file. Try \"task add [task]\" to create said file.");

    let mut lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    let lines_length: i32 = lines.len().try_into().expect("value error");
    let index_to_remove = ((task - lines_length) * (-1)) as usize;

    if index_to_remove >= lines.len() {
        eprintln!("No such task with that index number!");
    }

    let removed = lines.remove(index_to_remove);
    archive_removed(removed);

    let mut file = fs::File::create("data.txt").expect("Cannot open file");
    for line in lines {
        writeln!(file, "{}", line).expect("Cannot write to file");
    }
}

fn archive_removed(removed: String) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("archive.txt")
        .expect("Cannot open file");

    writeln!(file, "{}", removed).expect("Cannot write to file");
}