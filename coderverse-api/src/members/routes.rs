use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::members::handlers;

pub fn routes(db_pool: sqlx::PgPool) -> BoxedFilter<(impl Reply,)> {
    let db_pool = warp::any().map(move || db_pool.clone());
    let get_members_route = warp::path!("members")
        .and(warp::get())
        .and(db_pool.clone())
        .and_then(handlers::get_members);

    let get_member_route = warp::path!("members" / u64)
        .and(warp::get())
        .and(db_pool.clone())
        .and_then(handlers::get_member);

    let create_member_route = warp::path!("members")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_pool.clone())
        .and_then(handlers::create_member);

    let update_member_route = warp::path!("members" / u64)
        .and(warp::put())
        .and(warp::body::json())
        .and(db_pool.clone())
        .and_then(handlers::update_member);

    let delete_member_route = warp::path!("members" / u64)
        .and(warp::delete())
        .and(db_pool.clone())
        .and_then(handlers::delete_member);

    let upload_photo_route = warp::path!("members" / u64 / "photo")
        .and(warp::post())
        .and(warp::body::json())
        .and(db_pool.clone())
        .and_then(handlers::upload_photo);

    let routes = get_members_route
        .or(get_member_route)
        .or(create_member_route)
        .or(update_member_route)
        .or(delete_member_route)
        .or(upload_photo_route);

    routes.boxed()
}