use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub is_superuser: bool,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub name: String,
}
