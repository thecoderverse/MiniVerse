use crate::command::Command;
use crate::controller::Controller;
use crate::guide::show_guide;
use crate::view::View;
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
            let book = View::get_book();
            let result = Controller::insert(book);
            println!("{} kayıt eklendi", result);
        }
        Ok(Command::All) => {
            let books = Controller::get_all();
            View::list(books);
        }
        Ok(Command::Find(_)) => {
            let name = arguments[2].as_str();
            let books = Controller::find(name);
            View::list(books);
        }
        Ok(Command::List(_)) => {}
        Err(e) => {
            println!("{}", e);
        }
        _ => {}
    }
}
