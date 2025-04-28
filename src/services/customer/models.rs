use serde::{Deserialize, Serialize};
use sqlx::types::time::PrimitiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct AllCustomers {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCustomer {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCustomer {
    pub name: String,
    pub email: String,
}
