use crate::command::Command;
use crate::guide::show_guide;
use std::env;
use std::process::exit;
use std::str::FromStr;

mod command;
mod guide;
mod model;
mod test;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        show_guide();
        exit(1);
    }

    match Command::from_str(arguments[1].as_str()) {
        Ok(Command::Help) => {
            show_guide();
        }
        Err(e) => {
            println!("{}", e);
        }
        _ => {}
    }
}
