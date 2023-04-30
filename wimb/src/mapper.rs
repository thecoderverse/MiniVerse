use crate::model::Book;

pub fn get_authors(book: &Book) -> String {
    let mut authors = String::new();
    for a in book.authors.iter() {
        authors += format!("{},", a.0.as_str()).as_str();
    }
    authors
}

// pub fn convert_to_model(b: &books) -> BookModel {
//     BookModel::new(
//         b.title,
//         vec![],
//         b.publisher,
//         LocationModel::new(b.column, b.row, b.order),
//     )
// }
