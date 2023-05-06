use crate::command::{ListCommand, Order};
use crate::model::{BookInsert, BookSelect};
use crate::schema::books::dsl::books;
use crate::schema::books::{authors, publisher, title};
use diesel::{
    Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection, TextExpressionMethods,
};
use dotenvy::dotenv;
use std::env;

fn open_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL bilgisi eksik");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("{} veritabanına bağlanılamadı", database_url))
}

pub fn insert_book(book: &BookInsert) -> usize {
    let conn = &mut open_connection();
    diesel::insert_into(books)
        .values(book)
        .execute(conn)
        .expect("Db'ye kitap eklenirken hata oluştu.")
}

pub fn load_all_books() -> Vec<BookSelect> {
    let conn = &mut open_connection();
    books
        .load::<BookSelect>(conn)
        .expect("Kitaplar yüklenemedi")
}

pub fn load_books_by_order(args: ListCommand) -> Vec<BookSelect> {
    let conn = &mut open_connection();
    let count = args.count as i64;
    match args.field_name.as_str() {
        "title" => match args.order {
            Order::Asc => books
                .order(title.asc())
                .limit(count)
                .load::<BookSelect>(conn)
                .expect("sorgulama hatası"),
            Order::Desc => books
                .order(title.desc())
                .limit(count)
                .load::<BookSelect>(conn)
                .expect("sorgulama hatası"),
        },
        "authors" => match args.order {
            Order::Asc => books
                .order(authors.asc())
                .limit(count)
                .load::<BookSelect>(conn)
                .expect("sorgulama hatası"),
            Order::Desc => books
                .order(authors.desc())
                .limit(count)
                .load::<BookSelect>(conn)
                .expect("sorgulama hatası"),
        },
        "publisher" => match args.order {
            Order::Asc => books
                .order(publisher.asc())
                .limit(count)
                .load::<BookSelect>(conn)
                .expect("sorgulama hatası"),
            Order::Desc => books
                .order(publisher.desc())
                .limit(count)
                .load::<BookSelect>(conn)
                .expect("sorgulama hatası"),
        },
        _ => books
            .order(title.asc())
            .limit(count)
            .load::<BookSelect>(conn)
            .expect("sorgulama hatası"),
    }
}

pub fn find_books(book_name: &str) -> Vec<BookSelect> {
    let conn = &mut open_connection();
    let like_value = format!("%{}%", book_name);
    books
        .filter(title.like(like_value))
        .load::<BookSelect>(conn)
        .expect("Kitaplar bulunamadı")
}
