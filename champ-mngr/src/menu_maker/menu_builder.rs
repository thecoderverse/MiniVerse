use super::{
    menu_controller::MenuController, menu_entry::MenuEntry, menu_entry_builder::MenuEntryBuilder,
};

pub struct MenuBuilder {
    entries: Vec<Option<MenuEntry>>,
}

impl MenuBuilder {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn add_entry(&mut self, func: fn(MenuEntryBuilder) -> Vec<MenuEntry>) -> &mut Self {
        let entries = func(MenuEntryBuilder::new(self.entries.len() as u8, None));
        self.entries
            .extend(entries.into_iter().map(|entry| Some(entry)));
        self
    }

    pub fn build(&mut self) -> MenuController {
        MenuController::new(
            self.entries
                .iter_mut()
                .map(|entry| entry.take().unwrap())
                .collect(),
        )
    }
}
