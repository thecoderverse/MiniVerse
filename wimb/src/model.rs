use std::fmt::{Display, Formatter};

pub struct Location {
    column: i32,
    row: i32,
    order: i32,
}

pub struct Author(pub String);

pub struct Book {
    title: String,
    authors: Vec<Author>,
    publisher: String,
    location: Location,
}

impl Location {
    pub fn new(column: i32, row: i32, order: i32) -> Self {
        Self { column, row, order }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{}:{})", self.column, self.row, self.order)
    }
}

impl Book {
    pub fn new(title: String, authors: Vec<Author>, publisher: String, location: Location) -> Self {
        Self {
            title,
            authors,
            publisher,
            location,
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.title, self.location)
    }
}
