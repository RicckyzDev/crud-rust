use super::models::{AllCustomers, CreateCustomer, UpdateCustomer};
use crate::AppState;
use crate::services::auth::auth::AuthenticatedUser;
use crate::utils::response::success_response;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::{Pool, Postgres};
use time::PrimitiveDateTime;
use time::macros::format_description;

#[get("/customers")]
pub async fn fetch_all_customers(
    state: web::Data<AppState>,
    _auth: AuthenticatedUser,
) -> impl Responder {
    let customers = sqlx::query!("SELECT * FROM customers")
        .fetch_all(&state.postgres_client)
        .await;

    match customers {
        Ok(customers) => {
            let format = format_description!("[day]/[month]/[year] [hour]:[minute]:[second]"); // Formato BR

            let customers: Vec<AllCustomers> = customers
                .iter()
                .map(|customer| AllCustomers {
                    id: customer.id,
                    name: customer.name.clone(),
                    email: customer.email.clone(),
                    created_at: customer
                        .created_at
                        .as_ref() // Converte de Option para referência
                        .map(|dt| {
                            // Tenta formatar o timestamp, se falhar, retorna uma string padrão
                            dt.format(&format)
                                .unwrap_or_else(|_| "Erro ao formatar".to_string()) // Trata o erro de formatação
                        })
                        .unwrap_or_else(|| "Data inválida".to_string()), // Caso seja None, coloca uma data padrão
                })
                .collect();
            HttpResponse::Ok().json(customers)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
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
    cfg.service(fetch_all_customers);
    cfg.service(create_customer);
    cfg.service(update_customer);
}
