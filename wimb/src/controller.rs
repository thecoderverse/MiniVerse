use crate::db::insert_book;
use crate::model::{Book, BookInsert};

pub fn insert(book: Book) -> usize {
    let book_insert = BookInsert::from(book);
    insert_book(&book_insert)
}
