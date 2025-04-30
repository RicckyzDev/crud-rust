use crate::services::customer::models::{AllCustomers, CreateCustomer};
use serde::Serialize;
use time::{OffsetDateTime, macros::format_description};

#[derive(Debug, Serialize)]
pub struct CustomerResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

impl From<AllCustomers> for CustomerResponse {
    fn from(customer: AllCustomers) -> Self {
        let format = format_description!("[day]/[month]/[year] [hour]:[minute]:[second]");

        let created_at = customer
            .created_at
            .map(|dt| dt.format(&format).unwrap())
            .unwrap_or_else(|| "-".to_string());

        Self {
            id: customer.id,
            name: customer.name,
            email: customer.email,
            created_at,
        }
    }
}
