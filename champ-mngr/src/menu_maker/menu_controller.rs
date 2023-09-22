use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use crate::menu_maker::menu_entry_clicked_event::MenuEntryClickedEvent;

use super::menu_entry::MenuEntry;

#[derive(Default)]
pub struct MenuController {
    entries: Vec<MenuEntry>,
    navigation_stack: Vec<u8>,
}

impl MenuController {
    pub fn new(entries: Vec<MenuEntry>) -> Self {
        let mut controller = MenuController::default();
        controller.entries = entries;
        controller
    }

    pub fn wait(&mut self) {
        /*
            callbackler yerine buradan result dönebiliriz veya ikisi de kalabilir,
            menü içinde dolaşırken collapse ve expand için callback daha iyi
        */

        print!("Tercih: ");
        let _ = stdout().flush(); // üstteki printte \n olmadığı için yazdırmıyor?

        let mut input = String::new();
        let _ = stdin().read_line(&mut input);

        print!("\x1b[u");
        print!("\x1b[0J");

        let option = input.as_bytes()[0] as char;

        if let Some(entry) = self.entries.iter().find(|entry| entry.code() == option) {
            entry.callback()(
                MenuEntryClickedEvent {
                    entry_id: entry.id(),
                    entry_code: entry.code(),
                },
                self,
            );
        } else {
            println!("Geçerli olmayan bir seçenek seçtiniz!");
        }

        self.render();
    }

    pub fn render(&self) {
        self.entries
            .iter()
            .for_each(|entry| self.print_entry(&entry));
    }

    /*
        Ekrana yazdırma kısmı bu classın sorumluluğu değil gibi dışarıya taşınabilir?
    */
    fn print_entry(&self, entry: &MenuEntry) {
        if !self.is_entry_visible(entry) {
            return;
        }

        let is_code_visible =
            entry.is_always_visible() || self.navigation_stack.last() == entry.parent_id().as_ref();

        let is_expanded = self
            .navigation_stack
            .iter()
            .find(|id| entry.id() == **id)
            .is_some();

        let indent = self
            .navigation_stack
            .iter()
            .position(|id| entry.parent_id() == Some(*id))
            .map(|index| (index + 1) * 3)
            .unwrap_or(0);

        println!(
            "{:>indent$}[{}] {} {}",
            "",
            if is_code_visible { entry.code() } else { ' ' },
            if is_expanded { 'v' } else { '>' },
            entry.text()
        );
    }

    fn is_entry_visible(&self, entry: &MenuEntry) -> bool {
        entry.parent_id().is_none()
            || self
                .navigation_stack
                .iter()
                .find(|id| entry.parent_id().unwrap() == **id)
                .is_some()
    }

    pub fn expand(&mut self, entry_id: u8) {
        self.navigation_stack.push(entry_id);
    }

    pub fn collapse(&mut self) {
        if self.navigation_stack.is_empty() {
            // Böyle olmamalı sadece şimdilik burada
            exit(0);
        }

        self.navigation_stack.pop();
    }
}
