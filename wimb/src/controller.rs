use crate::command::ListCommand;
use crate::db::{delete_book, find_books, insert_book, load_all_books, load_books_by_order};
use crate::model::{Book, BookInsert, BookSelect};

pub struct Controller;

impl Controller {
    pub fn insert(book: Book) -> usize {
        let book_insert = BookInsert::from(book);
        insert_book(&book_insert)
    }

    pub fn find(title: &str) -> Vec<Book> {
        let books = find_books(title);
        Self::convert_books(books)
    }

    pub fn delete(book_id: i32) {
        delete_book(book_id);
    }

    pub fn get_by_order(args: ListCommand) -> Vec<Book> {
        let books = load_books_by_order(args);
        Self::convert_books(books)
    }

    pub fn get_all() -> Vec<Book> {
        let books = load_all_books();
        Self::convert_books(books)
    }

    fn convert_books(books: Vec<BookSelect>) -> Vec<Book> {
        let mut result = vec![];
        for book in books {
            result.push(Book::from(book));
        }
        result
    }
}
