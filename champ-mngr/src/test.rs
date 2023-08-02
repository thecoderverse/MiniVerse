#[cfg(test)]
mod test {
    use crate::ui::MenuBuilder;
    use crate::{Level, Manager};

    #[test]
    fn should_calculate_manager_bonus_test() {
        let morinyo = Manager::new("Hoze Muariniyo".to_string(), Level::Pro);
        let bonus = morinyo.calc_bonus();
        assert_eq!(bonus, 10);
    }

    #[test]
    fn should_menu_loaded_test() {
        let menu = MenuBuilder::load();
        assert_eq!(menu.len(), 4);
        assert_eq!(menu[3].code, 'X');
        assert_eq!(menu[2].parent_id, None);
    }
}
