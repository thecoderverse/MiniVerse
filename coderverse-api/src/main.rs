mod database;
mod members;

use warp::Filter;

#[tokio::main]
async fn main() {
    let db_pool = database::create_pool().await;
    let index_routes = warp::path::end().map(|| warp::reply::html("Hello, World!".to_owned()));
    let members_routes = members::routes::routes(db_pool);

    let routes = index_routes
        .or(members_routes)
        .with(warp::cors().allow_any_origin());

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}