use std::env;
use std::{fs, io};
use tosk::{self, create_file, DATA_FILE_PATH};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).map(String::as_str) == Some("help") {
        tosk::help();
        return;
    }

    match fs::read(&*DATA_FILE_PATH) {
        Ok(_) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            if let Err(e) = create_file() {
                eprintln!("Failed to create data file {:?}: {}", &*DATA_FILE_PATH, e);
                return;
            }
        }
        Err(e) => {
            eprintln!("Failed to access data file {:?}: {}", &*DATA_FILE_PATH, e);
            return;
        }
    }

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
            if args.len() >= 3 {
                let task = args[2..].join(" ");
                tosk::add(task);
            } else {
                eprintln!("Error: No task specified.");
            }
        }
        
        "rm" => {
            if args.len() >= 3 {
                if args[2] == "--all" || args[2] == "-all" {
                    tosk::rm_all();
                    return;
                } else if args[2].starts_with("-") {
                    eprintln!("Not a valid parameter. See \"tosk help\" for more information")
                    return;
                } else {
                    let mut valid_tasks = Vec::new();
                    for task in &args[2..] {
                        match task.trim().parse::<i32>() {
                            Ok(num) => valid_tasks.push(num),
                            Err(_) => eprintln!("\"{}\" is not a number!", task),
                        }
                    }
                    if !valid_tasks.is_empty() {
                        tosk::remove(valid_tasks);
                    }
                }
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
            } else if args.len() > 3{
                eprintln!("You can only edit one task at a time.")
            } else {
                eprintln!("Error: No task specified for edit");
            }
        }

        "reorder" | "move" | "reo"=> {
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
                if command.as_str() == "move" {
                    eprintln!("Error: Move takes 2 arguments!");
                } else {
                    eprintln!("Error: Reorder takes 2 arguments!");
                }
            }
        }

        _ => eprintln!("Unknown command: {}, see \"tosk help\" for more information", command),
    }
}
