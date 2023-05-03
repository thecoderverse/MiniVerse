use crate::schema::books;
use diesel::{Insertable, Queryable};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub struct Location {
    pub column: i16,
    pub row: i16,
    pub order: i16,
}

pub struct Author(pub String);

pub struct Book {
    pub title: String,
    pub authors: Vec<Author>,
    pub publisher: String,
    pub location: Location,
}

impl Location {
    pub fn new(column: i16, row: i16, order: i16) -> Self {
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

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct BookInsert {
    pub title: String,
    pub authors: String,
    pub publisher: String,
    pub column: i16,
    pub row: i16,
    pub order: i16,
}

#[derive(Queryable)]
#[diesel(table_name = books)]
pub struct BookSelect {
    pub id: i32,
    pub title: String,
    pub authors: String,
    pub publisher: String,
    pub column: i16,
    pub row: i16,
    pub order: i16,
}
