use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewMember {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UploadPhoto {
    pub photo_url: String,
}