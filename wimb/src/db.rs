use crate::mapper::get_authors;
use crate::model::BookModel;
use crate::schema::books::dsl::books;
use crate::schema::books::{authors, column, order, publisher, row, title};
use diesel::{Connection, ExpressionMethods, RunQueryDsl, SqliteConnection};
use dotenvy::dotenv;
use std::env;

pub fn open_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL bilgisi eksik");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("{} veritabanına bağlanılamadı", database_url))
}

pub fn insert_book(conn: &mut SqliteConnection, book: BookModel) -> usize {
    let author_names = get_authors(&book).to_string();

    diesel::insert_into(books)
        .values((
            title.eq(&book.title),
            publisher.eq(&book.publisher),
            authors.eq(author_names),
            column.eq(&book.location.column),
            row.eq(&book.location.row),
            order.eq(&book.location.order),
        ))
        .execute(conn)
        .expect("Db'ye kitap eklenirken hata oluştu.")
}
