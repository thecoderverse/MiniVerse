use super::{menu_controller::MenuController, menu_entry_clicked_event::MenuEntryClickedEvent};

/*
    Gizleyip gizlememek için bir seçenek eklenebilir
*/
pub struct MenuEntry {
    id: u8,
    code: char,
    text: String,
    parent_id: Option<u8>,
    is_always_visible: bool,
    callback: fn(MenuEntryClickedEvent, &mut MenuController),
}

impl MenuEntry {
    pub fn new(
        id: u8,
        code: char,
        text: String,
        parent_id: Option<u8>,
        is_always_visible: bool,
        callback: fn(MenuEntryClickedEvent, &mut MenuController),
    ) -> Self {
        Self {
            id,
            code,
            text: text,
            parent_id,
            is_always_visible,
            callback,
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn code(&self) -> char {
        self.code
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub fn parent_id(&self) -> Option<u8> {
        self.parent_id
    }

    pub fn is_always_visible(&self) -> bool {
        self.is_always_visible
    }

    pub fn callback(&self) -> fn(MenuEntryClickedEvent, &mut MenuController) {
        self.callback
    }
}
