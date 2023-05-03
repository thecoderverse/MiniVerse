// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Integer,
        title -> Text,
        authors -> Text,
        publisher -> Text,
        column -> SmallInt,
        row -> SmallInt,
        order -> SmallInt,
    }
}
