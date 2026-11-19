use std::{env, path::PathBuf};
use tosk;
use dirs::config_dir;
use std::fs;

fn main() {
    let os = std::env::consts::OS;
    let path = config_dir()
        .unwrap()
        .join(match os{
            "linux"=>{"tosk/"}
            "windows"=>{"tosk\\"}
            "macos"=>{"tosk/"}
            _ =>{unimplemented!("handle error or other os correctly")}
        });

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
        "list" |
        "ls" => {
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
                if task.starts_with("-") || task.starts_with("--"){
                    if task.contains("all") {
                        tosk::rm_all();
                    } else {
                        eprintln!("Not a valid parameter")
                    }
                } else {
                    match task.trim().parse::<i32>() {
                        Ok(num) => tosk::remove(num),
                        Err(_) => eprintln!("Not a number!"),
                    }
                }
            }
            else if args.len() > 3{
                eprintln!("You can only delete one task at a time.")
            } else {
                eprintln!("Error: No task specified for rm");
            }
        }

        "info" => {
            if args.len() == 3 {
                let task = &args[2];
                match task.trim().parse::<i32>() {
                    Ok(num) => tosk::info(num),
                    Err(_) => eprintln!("Not a number!"),
                }
            }
            else if args.len() > 3{
                eprintln!("You can only get information on one task at a time.")
            } else {
                eprintln!("Error: No task specified for info");
            }
        }

        "edit" => {
            if args.len() == 3 {
                let task = &args[2];
                match task.trim().parse::<i32>() {
                    Ok(num) => tosk::edit(num),
                    Err(_) => eprintln!("Not a number!"),
                }
            }
        }

        "reorder" => {
            if args.len() == 4 {
                let old_pos = &args[2];
                let new_pos = &args[3];

                match (
                    old_pos.trim().parse::<i32>(),
                    new_pos.trim().parse::<i32>()
                ) {
                    (Ok(old), Ok(new)) => {
                        tosk::reorder(old, new);
                    }
                    (Err(_), Err(_)) => {
                        println!("Wrong parameters, \"{}\" and \"{}\" are not numbers!", old_pos, new_pos);
                    }
                    (Err(_), _) => {
                        println!("Wrong first parameter, \"{}\" is not a number!", old_pos);
                    }
                    (_, Err(_)) => {
                        println!("Wrong second parameter, \"{}\" is not a number!", new_pos);
                    }
                }
            } else {
                eprintln!("Error: Reorder takes 2 arguments!");
            }
        }

        "help" => {
            tosk::help();
        }
        _ => eprintln!("Unknown command: {}, see \"tosk help\" for more information", command),
    }
}
