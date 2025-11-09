use helptext::{Help, sections};

const HELP: Help = Help(sections!(
    ["tosk " {env!("CARGO_PKG_VERSION")}]
    "USAGE" {
        ["tosk [OPTIONS] <INPUT>"]
    }
    "OPTIONS" {
        table Auto {
            "help" => {
                ["Print help information"]
            }
            "add <TASK>" => {
                ["Create a new task"]
            }
            "rm <index of task>" => {
                ["Delete / mark as completed a task"]
            }
            "rm --all" => {
                ["Delete / mark as completed all tasks"]
            }
            "list" => {
                ["List all tasks"]
            }
            "info <index of task>" => {
                ["Gives back information on the specified task"]
            }
        }
    }
));

pub fn text() {
    let _ = HELP.write(
        &mut std::io::stdout().lock(),
        false,
        false,
    );
}