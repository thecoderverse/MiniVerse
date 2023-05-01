use crate::model::{Author, Book, BookSelect, Location};

pub fn get_authors(book: &Book) -> String {
    let mut authors = String::new();
    for a in book.authors.iter() {
        authors += format!("{},", a.0.as_str()).as_str();
    }
    authors
}

pub fn get_authors_from_string(s: String) -> Vec<Author> {
    let parts: Vec<&str> = s.split(",").collect();
    let mut authors: Vec<Author> = Vec::new();
    for i in 0..parts.len() - 1 {
        authors.push(Author(parts[i].to_string()));
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
