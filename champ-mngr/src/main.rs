mod model;
mod test;
mod ui;

use crate::model::*;
use crate::ui::MenuBuilder;
use colorized::{Color, Colors};
use std::io::stdin;

fn main() {
    let manager = Manager::new("Haymi Mendirali".to_string(), Level::Amateur);
    let _game_state = GameState { current_score: 0 };
    let menu = MenuBuilder::load();
    let welcome_message = format!("Hoşgelin '{}'", manager).color(Colors::CyanFg);
    println!("{}", welcome_message);
    loop {
        menu.iter().for_each(|m| println!("{}", m));

        let mut answer = String::new();
        let _ = stdin().read_line(&mut answer);
        //println!("Şöyle dedin {}", answer);
        let answer = answer.trim();

        if answer == "X" {
            println!("Hoşçakal");
            break;
        } else if answer == "0" {
            println!("Yeni sezon açılış menüsü");
        } else if answer == "1" {
            println!("Önceki sezon yüklenecek...");
        } else if answer == "2" {
            println!("Ayarlar menüsü gelecek...");
        } else {
            println!("Üzgünüm ama ne dediğini anlayamadım");
        }
    }
}
