use super::{
    menu_controller::MenuController, menu_entry::MenuEntry,
    menu_entry_clicked_event::MenuEntryClickedEvent,
};

#[derive(Default)]
pub struct MenuEntryBuilder {
    id: u8,
    code: Option<char>,
    text: Option<String>,
    parent_id: Option<u8>,
    is_always_visible: bool,
    callback: Option<fn(MenuEntryClickedEvent, &mut MenuController)>,
    subentries: Vec<Option<MenuEntry>>,
}

impl MenuEntryBuilder {
    pub fn new(id: u8, parent_id: Option<u8>) -> Self {
        let mut builder = Self::default();
        builder.id = id;
        builder.parent_id = parent_id;
        builder
    }

    pub fn code(&mut self, code: char) -> &mut Self {
        self.code = Some(code);
        self
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text = Some(text.to_owned());
        self
    }

    pub fn always_visible(&mut self) -> &mut Self {
        self.is_always_visible = true;
        self
    }

    pub fn callback(
        &mut self,
        callback: fn(MenuEntryClickedEvent, &mut MenuController),
    ) -> &mut Self {
        self.callback = Some(callback);
        self
    }

    pub fn add_subentry(&mut self, func: fn(MenuEntryBuilder) -> Vec<MenuEntry>) -> &mut Self {
        let entries = func(MenuEntryBuilder::new(
            self.id + 1 + self.subentries.len() as u8,
            Some(self.id),
        ));
        self.subentries
            .extend(entries.into_iter().map(|entry| Some(entry)));
        self
    }

    pub fn build(&mut self) -> Vec<MenuEntry> {
        self.subentries.insert(
            0,
            Some(MenuEntry::new(
                self.id,
                self.code.unwrap(),
                self.text.take().unwrap(),
                self.parent_id,
                self.is_always_visible,
                self.callback.unwrap(),
            )),
        );

        self.subentries
            .iter_mut()
            .map(|entry| entry.take().unwrap())
            .collect()
    }
}
