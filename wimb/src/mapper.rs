use crate::model::{Author, Book, BookInsert, BookSelect, Location};

pub fn get_authors(source: Vec<Author>) -> String {
    let mut authors = String::new();
    for a in source.iter() {
        authors += format!("{},", a.0.as_str()).as_str();
    }
    authors
}

pub fn get_authors_from_string(s: String) -> Vec<Author> {
    let parts: Vec<&str> = s.split(',').collect();
    let mut authors: Vec<Author> = Vec::new();
    for a in parts.iter().take(parts.len() - 1) {
        authors.push(Author(a.to_string()));
    }
    authors
}

impl From<BookSelect> for Book {
    fn from(value: BookSelect) -> Self {
        let location = Location {
            column: value.column,
            row: value.row,
            order: value.order,
        };

        Self {
            title: value.title,
            publisher: value.publisher,
            location,
            authors: get_authors_from_string(value.authors),
        }
    }
}

impl From<Book> for BookInsert {
    fn from(value: Book) -> Self {
        Self {
            title: value.title,
            authors: get_authors(value.authors),
            publisher: value.publisher,
            column: value.location.column,
            row: value.location.row,
            order: value.location.order,
        }
    }
}
