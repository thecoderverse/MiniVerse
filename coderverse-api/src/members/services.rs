use crate::members::models::{Member, NewMember};

use super::models::UploadPhoto;

pub async fn get_member(id: u64, db_pool: &sqlx::PgPool) -> Result<Member, sqlx::Error> {
    let id = id as i32;
    let member = sqlx::query_as!(Member, r#"SELECT id, name, email, photo_url FROM members WHERE id = $1"#, id)
        .fetch_one(db_pool)
        .await;

    match member {
        Err(e) => return Err(e),
        Ok(member) => return Ok(member),
    }

}

pub async fn get_members(db_pool: &sqlx::PgPool) -> Result<Vec<Member>, sqlx::Error> {
    let members = sqlx::query_as!(Member, "SELECT id, name, email, photo_url FROM members")
        .fetch_all(db_pool)
        .await;

    match members {
        Err(e) => return Err(e),
        Ok(members) => return Ok(members),
    }
}

pub async fn create_member(member: NewMember, db_pool: &sqlx::PgPool) -> Result<Member, sqlx::Error> {
    let member = sqlx::query_as!(Member, r#"INSERT INTO members (name, email) VALUES ($1, $2) RETURNING *"#, member.name, member.email)
        .fetch_one(db_pool)
        .await;

    match member {
        Err(e) => return Err(e),
        Ok(member) => return Ok(member),
    }
}

pub async fn update_member(id: u64, member: Member, db_pool: &sqlx::PgPool) -> Result<Member, sqlx::Error> {
    let id = id as i32;
    let member = sqlx::query_as!(Member, r#"UPDATE members SET name = $1, email = $2 WHERE id = $3 RETURNING *"#, member.name, member.email, id)
        .fetch_one(db_pool)
        .await;

    match member {
        Err(e) => return Err(e),
        Ok(member) => return Ok(member),
    }
}

pub async fn delete_member(id: u64, db_pool: &sqlx::PgPool) -> Result<Member, sqlx::Error> {
    let id = id as i32;
    let member = sqlx::query_as!(Member, r#"DELETE FROM members WHERE id = $1 RETURNING *"#, id)
        .fetch_one(db_pool)
        .await;

    match member {
        Err(e) => return Err(e),
        Ok(member) => return Ok(member)
    }
}

pub async fn upload_photo(id: u64, photo_url: UploadPhoto, db_pool: &sqlx::PgPool) -> Result<Member, sqlx::Error> {
    let id = id as i32;
    let member = sqlx::query_as!(Member, r#"UPDATE members SET photo_url = $1 WHERE id = $2 RETURNING *"#, photo_url.photo_url, id)
        .fetch_one(db_pool)
        .await;

    match member {
        Err(e) => return Err(e),
        Ok(member) => return Ok(member)
    }
}