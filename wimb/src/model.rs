use std::fmt::{Display, Formatter};
use diesel::Queryable;

pub struct LocationModel {
    pub column: i16,
    pub row: i16,
    pub order: i16,
}

pub struct AuthorModel(pub String);

#[derive(Queryable)]
pub struct BookModel {
    pub title: String,
    pub authors: Vec<AuthorModel>,
    pub publisher: String,
    pub location: LocationModel,
}

impl LocationModel {
    pub fn new(column: i16, row: i16, order: i16) -> Self {
        Self { column, row, order }
    }
}

impl Display for LocationModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{}:{})", self.column, self.row, self.order)
    }
}

impl BookModel {
    pub fn new(
        title: String,
        authors: Vec<AuthorModel>,
        publisher: String,
        location: LocationModel,
    ) -> Self {
        Self {
            title,
            authors,
            publisher,
            location,
        }
    }
}

impl Display for BookModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.title, self.location)
    }
}

// #[derive(Insertable)]
// #[diesel(table_name=books)]
// pub struct Book {
//     pub id: i32,
//     pub title: String,
//     pub authors: String,
//     pub publisher: String,
//     pub column: i16,
//     pub row: i16,
//     pub order: i16,
// }
