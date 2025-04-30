use super::dto::CustomerResponse;
use super::models::{AllCustomers, CreateCustomer, FilterOptions, UpdateCustomer};
use crate::services::auth::auth::AuthenticatedUser;
use crate::utils::response::success_response;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::{Pool, Postgres};
use time::macros::format_description;
use time::PrimitiveDateTime;

#[get("/customers")]
pub async fn fetch_all_customers(
    opts: web::Query<FilterOptions>,
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        AllCustomers,
        "SELECT * FROM customers ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&state.postgres_client)
    .await
    {
        Ok(customers) => {
            let total_count = sqlx::query!("SELECT COUNT(*) as count FROM customers")
                .fetch_one(&state.postgres_client)
                .await
                .unwrap()
                .count;

            // 🚀 Aqui converte todos os AllCustomers para CustomerResponse
            let customers: Vec<CustomerResponse> =
                customers.into_iter().map(CustomerResponse::from).collect();

            let response = json!({
                "total_count": total_count,
                "customers": customers,
            });

            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/customers/{id}")]
pub async fn fetch_customer_by_id(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let customer = sqlx::query_as!(
        AllCustomers,
        "SELECT * FROM customers WHERE id = $1",
        id.into_inner()
    )
    .fetch_one(&state.postgres_client)
    .await;

    match customer {
        Ok(customer) => HttpResponse::Ok().json(CustomerResponse::from(customer)),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[post("/customers")]
pub async fn create_customer(
    state: web::Data<AppState>,
    new_customer: web::Json<CreateCustomer>,
) -> impl Responder {
    let customer = sqlx::query!(
        "INSERT INTO customers (name, email) VALUES ($1, $2) RETURNING id",
        new_customer.name,
        new_customer.email
    )
    .fetch_one(&state.postgres_client)
    .await;

    match customer {
        Ok(_customer) => success_response("Customer created successfully", 201),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/customers/{id}")]
pub async fn update_customer(
    state: web::Data<AppState>,
    customer: web::Json<UpdateCustomer>,
    id: web::Path<i32>,
) -> impl Responder {
    let customer = sqlx::query!(
        "UPDATE customers SET name = $1, email = $2 WHERE id = $3",
        customer.name,
        customer.email,
        id.into_inner()
    )
    .execute(&state.postgres_client)
    .await;

    match customer {
        Ok(_customer) => success_response("Customer updated successfully", 200),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn config_customers_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api");
    cfg.service(
        scope
            .service(fetch_all_customers)
            .service(fetch_customer_by_id)
            .service(create_customer)
            .service(update_customer),
    );
}
