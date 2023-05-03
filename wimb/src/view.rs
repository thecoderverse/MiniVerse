use crate::model::{Author, Book, Location};
use std::io::stdin;
use std::str::FromStr;

pub fn add() -> Book {
    println!("Kitabın Adı ");
    let mut book_name = String::new();
    stdin().read_line(&mut book_name).expect("girdi alınamadı!");

    println!("Yazar sayısı ");
    let mut authors_count = String::new();
    stdin()
        .read_line(&mut authors_count)
        .expect("girdi alınamadı");
    let count = i16::from_str(authors_count.trim()).expect("sayısal değil");
    let mut authors = Vec::new();
    for i in 0..count {
        println!("{}. yazar ", i);
        let mut author_name = String::new();
        stdin()
            .read_line(&mut author_name)
            .expect("girdi alınamadı!");
        authors.push(Author(author_name.trim().to_string()));
    }

    println!("Yayıncı ");
    let mut publisher = String::new();
    stdin().read_line(&mut publisher).expect("girdi alınamadı!");

    println!("Blok(Column)");
    let mut column = String::new();
    stdin().read_line(&mut column).expect("girdi alınamadı!");
    let clm = i16::from_str(column.trim()).expect("sayısal değer değil");

    println!("Raf(Row)");
    let mut row = String::new();
    stdin().read_line(&mut row).expect("girdi alınamadı!");
    let rw = i16::from_str(row.trim()).expect("sayısal değer değil");

    println!("Sıra(Order)");
    let mut order = String::new();
    stdin().read_line(&mut order).expect("girdi alınamadı!");
    let ord = i16::from_str(order.trim()).expect("sayısal değer değil");

    let book = Book {
        title: book_name.trim().to_string(),
        authors,
        publisher: publisher.trim().to_string(),
        location: Location::new(clm, rw, ord),
    };
    book
}
