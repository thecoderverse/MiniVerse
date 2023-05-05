use crate::db::{find_books, insert_book};
use crate::model::{Book, BookInsert};

pub struct Controller;

impl Controller {
    pub fn insert(book: Book) -> usize {
        let book_insert = BookInsert::from(book);
        insert_book(&book_insert)
    }

    pub fn find(title: &str) -> Vec<Book> {
        let books = find_books(title);
        let mut result = vec![];
        for book in books {
            result.push(Book::from(book));
        }
        result
    }
}
