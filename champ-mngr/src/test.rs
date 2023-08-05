#[cfg(test)]
mod test {
    use crate::{Level, Manager};

    #[test]
    fn should_calculate_manager_bonus_test() {
        let morinyo = Manager::new("Hoze Muariniyo".to_string(), Level::Pro);
        let bonus = morinyo.calc_bonus();
        assert_eq!(bonus, 10);
    }
}
