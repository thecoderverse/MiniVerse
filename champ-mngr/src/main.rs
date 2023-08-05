mod menu;
mod menu_maker;
mod model;
mod test;

use crate::model::*;
use colorized::{Color, Colors};

fn main() {
    let manager = Manager::new("Haymi Mendirali".to_string(), Level::Amateur);
    let _game_state = GameState { current_score: 0 };
    let mut menu = menu::load_menu();
    let welcome_message = format!("Hoşgelin '{}'", manager).color(Colors::CyanFg);
    println!("{}", welcome_message);

    print!("\x1b[s");
    menu.render();

    loop {
        menu.wait();
    }
}
