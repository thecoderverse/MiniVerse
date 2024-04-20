mod handlers;

use warp::{filters::cors::Builder, Filter};
use handlers::note::{add_note_handler, get_note_handler, delete_note_handler, get_note_by_id_handler};
use handlers::index::index_handler;

pub fn get_routes(cors: Builder) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET /
    let index_route = warp::path::end()
        .and(warp::get())
        .and_then(index_handler);

    // GET /note
    let get_note_route = warp::path("note")
        .and(warp::get())
        .and_then(get_note_handler);

    // GET /note/{id}
    let get_note_by_id_route = warp::path("note")
        .and(warp::path::param())
        .and(warp::get())
        .and_then(get_note_by_id_handler);

    // POST /note/create
    let add_note_route = warp::path("note")
        .and(warp::path("create"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add_note_handler);

    // DELETE /note/delete/{id}
    let delete_note_route = warp::path("note")
        .and(warp::path("delete"))
        .and(warp::delete())
        .and(warp::path::param())
        .and_then(delete_note_handler);

    let routes = index_route
        .or(get_note_by_id_route)
        .or(get_note_route)
        .or(add_note_route)
        .or(delete_note_route)
        .with(cors);

    routes
}