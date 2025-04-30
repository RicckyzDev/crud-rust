use super::models::{AllUsers, JsonWebTokenClaims, LoginUser, RegisterUser, UpdateUser};
use crate::services::auth::auth::AuthenticatedUser;
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use serde_json::json;
use sqlx::{Pool, Postgres};

#[get("/users")]
async fn get_all_users(state: web::Data<AppState>, _auth: AuthenticatedUser) -> impl Responder {
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

#[post("/users/login")]
async fn login_user(state: web::Data<AppState>, body: web::Json<LoginUser>) -> impl Responder {
    if body.email.is_empty() || body.password.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    let result = sqlx::query!("SELECT * FROM users WHERE email = $1", body.email)
        .fetch_one(&state.postgres_client)
        .await;

    match result {
        Ok(user) => {
            let password_valid = bcrypt::verify(body.password.clone(), &user.password)
                .expect("Error verifying password");

            if password_valid {
                let claims = JsonWebTokenClaims {
                    sub: user.id,
                    exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
                    name: user.username,
                    email: user.email,
                };

                let token = jsonwebtoken::encode(
                    &jsonwebtoken::Header::default(),
                    &claims,
                    &jsonwebtoken::EncodingKey::from_secret(state.json_web_token.as_ref()),
                )
                .expect("Error creating token");

                return HttpResponse::Ok().json(json!({
                    "data": token
                }));
            } else {
                HttpResponse::NotFound().json(json!({"message": "User not found" }))
            }
        }
        Err(_) => HttpResponse::NotFound().json(json!({
            "message": "User not found"
        })),
    }
}

pub fn config_users_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api");
    cfg.service(
        scope
            .service(get_all_users)
            .service(register_user)
            .service(update_user)
            .service(delete_user)
            .service(login_user),
    );
}
