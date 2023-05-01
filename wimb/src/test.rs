#[cfg(test)]
mod test {
    use crate::command::{Command, ListCommand, ListCommandParseError, Order, ParseError};
    use crate::db::{find_books, insert_book, load_all_books, open_connection};
    use crate::mapper::{get_authors, get_authors_from_string};
    use crate::model::{Author, Book, BookInsert, BookSelect, Location};
    use std::str::FromStr;

    #[test]
    fn should_from_string_to_asc_order_works() {
        let ordering = "asc";
        let actual = Order::from_str(ordering);
        assert_eq!(actual, Ok(Order::Asc));
    }

    #[test]
    fn should_from_string_to_desc_order_works() {
        let ordering = "desc";
        let actual = Order::from_str(ordering);
        assert_eq!(actual, Ok(Order::Desc));
    }

    #[test]
    fn should_not_convert_to_any_order_works() {
        let ordering = "ascendig";
        let actual = Order::from_str(ordering);
        assert!(actual.is_err());
    }

    #[test]
    fn should_convert_to_add_works() {
        let command = "-add";
        let actual = Command::from_str(command);
        assert_eq!(actual, Ok(Command::Add));
    }

    #[test]
    fn should_convert_to_find_works() {
        let command = "-find";
        let actual = Command::from_str(command);
        assert_eq!(actual, Ok(Command::Find(String::default())));
    }

    #[test]
    fn should_convert_to_del_works() {
        let command = "-del";
        let actual = Command::from_str(command);
        assert_eq!(actual, Ok(Command::Del(u32::default())));
    }

    #[test]
    fn should_convert_to_help_works() {
        let command = "-help";
        let actual = Command::from_str(command);
        assert_eq!(actual, Ok(Command::Help));
    }

    #[test]
    fn should_convert_to_list_works() {
        let command = "-list";
        let actual = Command::from_str(command);
        assert_eq!(actual, Ok(Command::List(ListCommand::default())));
    }

    #[test]
    fn should_not_convert_to_any_command_works() {
        let command = "-unknown";
        let actual = Command::from_str(command);
        assert_eq!(actual, Err(ParseError::Command));
    }

    #[test]
    fn should_not_convert_to_any_list_command_with_wrong_number_in_parameters_works() {
        let list_command_parms = "name desc 10 top";
        let actual = ListCommand::from_str(list_command_parms);
        assert_eq!(actual, Err(ListCommandParseError::WrongParameterCount));
    }

    #[test]
    fn should_not_convert_to_any_list_command_with_invalid_ordering_works() {
        let list_command_parms = "name blabla 10";
        let actual = ListCommand::from_str(list_command_parms);
        assert_eq!(actual, Err(ListCommandParseError::InvalidOrdering));
    }

    #[test]
    fn should_not_convert_to_any_list_command_with_invalid_number_works() {
        let list_command_parms = "name desc blabla";
        let actual = ListCommand::from_str(list_command_parms);
        assert_eq!(actual, Err(ListCommandParseError::InvalidNumber));
    }

    #[test]
    fn should_convert_to_list_command_works() {
        let list_command_parms = "name desc 10";
        let actual = ListCommand::from_str(list_command_parms);
        assert_eq!(
            actual,
            Ok(ListCommand::init("name".to_string(), Order::Desc, 10))
        );
    }

    #[test]
    fn should_create_a_book_works() {
        let authors = vec![
            Author("Saurabh Shrivastava".to_string()),
            Author("Neenlanjali Srivastav".to_string()),
        ];
        let book = Book::new(
            "Solutions Architect's Handbook".to_string(),
            authors,
            "Packt".to_string(),
            Location::new(2, 4, 8),
        );
        assert_eq!(book.to_string(), "Solutions Architect's Handbook,(2:4:8)");
    }

    #[test]
    fn should_authors_convert_to_single_string_works() {
        let book = Book::new(
            "Essential Windows Communication Foundation for .Net Framework 3.5".to_string(),
            vec![
                Author("Steve Resnick".to_string()),
                Author("Richard Crane".to_string()),
                Author("Chris Bowen".to_string()),
            ],
            "Packt Publishing".to_string(),
            Location::new(1, 2, 3),
        );
        let authors = get_authors(&book);
        assert_eq!(authors, "Steve Resnick,Richard Crane,Chris Bowen,");
    }

    #[test]
    fn should_insert_from_model_to_db_works() {
        let conn = &mut open_connection();
        let book_model = Book::new(
            "Essential Windows Communication Foundation for .Net Framework 3.5".to_string(),
            vec![
                Author("Steve Resnick".to_string()),
                Author("Richard Crane".to_string()),
                Author("Chris Bowen".to_string()),
            ],
            "Packt Publishing".to_string(),
            Location::new(1, 2, 3),
        );
        let author_names = get_authors(&book_model);
        let book = BookInsert {
            title: book_model.title,
            authors: author_names,
            publisher: book_model.publisher,
            column: book_model.location.column,
            row: book_model.location.row,
            order: book_model.location.order,
        };
        let inserted = insert_book(conn, &book);
        assert_eq!(inserted, 1);
    }

    #[test]
    fn should_load_all_books_works() {
        let conn = &mut open_connection();
        let expected = load_all_books(conn);
        assert!(expected.len() > 0);
    }

    #[test]
    fn should_filter_books_by_name_works() {
        let conn = &mut open_connection();
        let expected = find_books(conn, "Windows".to_string());
        assert!(expected.len() > 0);
    }

    #[test]
    fn should_get_authors_from_string_works() {
        let source = "Steve Resnick,Richard Crane,Packt Publishing,".to_string();
        let expected = get_authors_from_string(source);
        assert_eq!(expected.len(), 3);
        assert_eq!(expected[0].0, "Steve Resnick");
        assert_eq!(expected[1].0, "Richard Crane");
        assert_eq!(expected[2].0, "Packt Publishing");
    }

    #[test]
    fn should_from_bookselect_to_book_mapper_works() {
        let source = BookSelect {
            id: 19,
            title: "Essential Windows Communication Foundation for .Net Framework 3.5".to_string(),
            authors: "Steve Resnick,Richard Crane,Packt Publishing,".to_string(),
            publisher: "Packt Publishing".to_string(),
            column: 1,
            row: 2,
            order: 3,
        };

        let book = Book::from(source);
        assert_eq!(
            book.title,
            "Essential Windows Communication Foundation for .Net Framework 3.5"
        );
        assert_eq!(book.location, Location::new(1, 2, 3));
        assert_eq!(book.authors.len(), 3);
        assert_eq!(book.authors[0].0, "Steve Resnick");
        assert_eq!(book.publisher, "Packt Publishing");
    }
}
