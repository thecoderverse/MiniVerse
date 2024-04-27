use crate::members::services;

pub async fn get_member(id: u64, db_pool: sqlx::PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    let member =    
    services::get_member(id, &db_pool).await.unwrap();
    Ok(warp::reply::json(&member))
}

pub async fn get_members(db_pool: sqlx::PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    let members = services::get_members(&db_pool).await.unwrap();
    Ok(warp::reply::json(&members))
}

pub async fn create_member(member: crate::members::models::NewMember, db_pool: sqlx::PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    let member = services::create_member(member, &db_pool).await.unwrap();
    Ok(warp::reply::json(&member))
}

pub async fn update_member(id: u64, member: crate::members::models::Member, db_pool: sqlx::PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    let member = services::update_member(id, member, &db_pool).await.unwrap();
    Ok(warp::reply::json(&member))
}

pub async fn delete_member(id: u64, db_pool: sqlx::PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    let member = services::delete_member(id, &db_pool).await.unwrap();
    Ok(warp::reply::json(&member))
}

pub async fn upload_photo(id: u64, photo: crate::members::models::UploadPhoto, db_pool: sqlx::PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    let member = services::upload_photo(id, photo, &db_pool).await.unwrap();
    Ok(warp::reply::json(&member))
}