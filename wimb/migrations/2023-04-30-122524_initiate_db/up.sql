create table if not exists books(
    id integer primary key autoincrement ,
    title text not null,
    authors text not null,
    publisher text not null,
    [column] smallint not null,
    [row] smallint not null,
    [order] smallint not null
);