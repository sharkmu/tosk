use dotenv::dotenv;
use std::env;

pub fn load() -> bool{
    dotenv().ok();

    let mut archive: bool = true;

    match env::var("ARCHIVE_ENABLED") {
        Ok(val) => {
            match val.parse::<bool>() {
                Ok(b) => archive = b,
                Err(_) => {
                    println!("ARCHIVE_ENABLED is not a bool.");
                }
            }
        }
        Err(_) => {
            println!("ARCHIVE_ENABLED is not set.");
        }
    }

    archive
}