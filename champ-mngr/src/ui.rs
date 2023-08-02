use std::fmt::{Display, Formatter};

pub struct Menu {
    pub id: u8,
    pub title: String,
    pub code: char,
    pub parent_id: Option<u8>,
}
impl Display for Menu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] - {}", self.code, self.title)
    }
}

// pub struct SubMenu {
//     pub id: u8,
//     pub menu_id: u8,
//     pub title: String,
//     pub code: u8,
// }

pub struct MenuBuilder;

impl MenuBuilder {
    pub fn load() -> Vec<Menu> {
        vec![
            Menu {
                id: 0,
                title: "Yeni Sezon".to_string(),
                code: '0',
                parent_id: None,
            },
            Menu {
                id: 1,
                title: "Öncekinden Devam Et".to_string(),
                code: '1',
                parent_id: None,
            },
            Menu {
                id: 2,
                title: "Ayarlar".to_string(),
                code: '2',
                parent_id: None,
            },
            Menu {
                id: 3,
                title: "Çıkış".to_string(),
                code: 'X',
                parent_id: None,
            },
        ]
    }
}
