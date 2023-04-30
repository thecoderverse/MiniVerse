use crate::model::{BookInsert, BookSelect};
use crate::schema::books::dsl::books;
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use dotenvy::dotenv;
use std::env;

pub fn open_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL bilgisi eksik");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("{} veritabanına bağlanılamadı", database_url))
}

pub fn insert_book(conn: &mut SqliteConnection, book: &BookInsert) -> usize {
    diesel::insert_into(books)
        .values(book)
        .execute(conn)
        .expect("Db'ye kitap eklenirken hata oluştu.")
}

pub fn load_all_books(conn: &mut SqliteConnection) -> Vec<BookSelect> {
    books
        .load::<BookSelect>(conn)
        .expect("Kitaplar yüklenemedi")
}
