use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AllUsers {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonWebTokenClaims {
    pub sub: i32,
    pub exp: usize,
    pub name: String,
    pub email: String,
    pub user_id: i32,
    pub company_id: i64,
}
