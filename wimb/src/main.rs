use crate::command::Command;
use crate::controller::insert;
use crate::guide::show_guide;
use crate::view::add;
use std::env;
use std::process::exit;
use std::str::FromStr;

mod command;
mod controller;
mod db;
mod guide;
mod mapper;
mod model;
mod schema;
mod test;
mod view;

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
        Ok(Command::Add) => {
            let book = add();
            let result = insert(book);
            println!("{} kayıt eklendi", result);
        }
        Err(e) => {
            println!("{}", e);
        }
        _ => {}
    }
}
