use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow)]
pub struct AllCustomers {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: Option<OffsetDateTime>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
