use std::{env, path::PathBuf};
use tosk;
use dirs::config_dir;
use std::fs;

fn main() {
    let path = config_dir()
        .unwrap()
        .join("tosk\\");

    match fs::read_dir(&path) {
        Ok(_) => handle_args(),
        Err(_) => create_folder(&path),
    }
}

fn create_folder(path: &PathBuf) {
    if let Err(e) = fs::create_dir_all(path) {
        eprintln!("Failed to create folder {:?}: {}", path, e);
    } else {
        handle_args();
    }
}

fn handle_args() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No args specified! Run \"tosk help\" for information regarding usage.");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "list" => {
            tosk::list();
        }
        "add" => {
            if args.len() == 3 {
                let task = &args[2];
                tosk::add(task.to_string());
            } 
            else if args.len() > 3 {
                let task = args[2..].join(" ");
                tosk::add(task);
            } else {
                eprintln!("Error: No task specified.");
            }
        }
        "rm" => {
            if args.len() == 3 {
                let task = &args[2];
                match task.trim().parse::<i32>() {
                    Ok(num) => tosk::remove(num),
                    Err(_) => eprintln!("Not a number!"),
                }
            }
            else if args.len() > 3{
                eprintln!("You can only delete one task at a time.")
            } else {
                eprintln!("Error: No task specified for rm");
            }
        }
        "help" => {
            tosk::help();
        }
        _ => eprintln!("Unknown command: {}, see \"tosk help\" for more information", command),
    }
}