use crate::menu_maker::{menu_builder::MenuBuilder, menu_controller::MenuController};

pub fn load_menu() -> MenuController {
    let mut menu_builder = MenuBuilder::new();

    menu_builder
        .add_entry(|mut entry_builder| {
            entry_builder
                .code('0')
                .text("Yeni Sezon")
                .callback(|event, _| println!("{:?}", event))
                .build()
        })
        .add_entry(|mut entry_builder| {
            entry_builder
                .code('1')
                .text("Öncekinden Devam Et")
                .callback(|event, _| println!("{:?}", event))
                .build()
        })
        .add_entry(|mut entry_builder| {
            entry_builder
                .code('2')
                .text("Ayarlar")
                .callback(|event, controller| controller.expand(event.entry_id))
                .add_subentry(|mut entry_builder| {
                    entry_builder
                        .code('S')
                        .text("Ses Seviyesi")
                        .callback(|event, _| println!("{:?}", event))
                        .build()
                })
                .add_subentry(|mut entry_builder| {
                    entry_builder
                        .code('Z')
                        .text("Zorluk Seviyesi")
                        .callback(|event, controller| controller.expand(event.entry_id))
                        .add_subentry(|mut entry_builder| {
                            entry_builder
                                .code('E')
                                .text("Kolay")
                                .callback(|event, _| println!("{:?}", event))
                                .build()
                        })
                        .add_subentry(|mut entry_builder| {
                            entry_builder
                                .code('M')
                                .text("Orta")
                                .callback(|event, _| println!("{:?}", event))
                                .build()
                        })
                        .add_subentry(|mut entry_builder| {
                            entry_builder
                                .code('H')
                                .text("Zor")
                                .callback(|event, _| println!("{:?}", event))
                                .build()
                        })
                        .build()
                })
                .build()
        })
        .add_entry(|mut entry_builder| {
            entry_builder
                .code('B')
                .text("Geri")
                .always_visible()
                .callback(|_, controller| controller.collapse())
                .build()
        })
        .build()
}
