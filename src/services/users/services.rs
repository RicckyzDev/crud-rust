use super::models::{AllUsers, RegisterUser, UpdateUser};
use crate::AppState;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use sqlx::{Pool, Postgres};

#[get("/users")]
async fn get_all_users(state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("SELECT * FROM users")
        .fetch_all(&state.postgres_client)
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(
            users
                .into_iter()
                .map(|user| AllUsers {
                    id: user.id,
                    name: user.username.clone(),
                    email: user.email.clone(),
                })
                .collect::<Vec<AllUsers>>(),
        ),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/users")]
async fn register_user(
    state: web::Data<AppState>,
    user: web::Json<RegisterUser>,
) -> impl Responder {
    let user = user.into_inner(); // pega a struct propriamente dita
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id, username, email",
        user.name,
        user.email,
        hashed_password
    )
    .fetch_one(&state.postgres_client)
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(AllUsers {
            id: user.id,
            name: user.username,
            email: user.email,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/users/{id}")]
async fn update_user(
    state: web::Data<AppState>,
    user: web::Json<UpdateUser>,
    id: web::Path<i32>,
) -> impl Responder {
    let user = user.into_inner();
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "UPDATE users SET username = $1, email = $2, password = $3 WHERE id = $4 RETURNING id, username, email",
        user.name,
        user.email,
        hashed_password,
        id.into_inner()
    )
    .fetch_one(&state.postgres_client)
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(AllUsers {
            id: user.id,
            name: user.username,
            email: user.email,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/users/{id}")]
async fn delete_user(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id.into_inner())
        .execute(&state.postgres_client)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn config_users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(register_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}
