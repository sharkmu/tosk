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
            "list (ls)" => {
                ["List all tasks"]
            }
            "info <index of task>" => {
                ["Gives back information on the specified task"]
            }
            "edit <index of task>" => {
                ["Prints the old content of the task and saves the new"]
            }
            "reorder <old index> <new index>" => {
                ["Takes the task at the old index and puts it in the new index"]
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
