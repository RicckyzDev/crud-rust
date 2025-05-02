use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi, ToSchema,
};

use utoipa_swagger_ui::SwaggerUi;

mod databases {
    pub mod postgres_connection;
}

mod exceptions;
mod services;
mod utils;

#[derive(Clone)]
pub struct AppState {
    postgres_client: Pool<Postgres>,
    json_web_token: String,
}

#[get("/")] // <-- Adicione a rota aqui (falta o "/" no seu código)
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info) // Nível mínimo de log
        .format_timestamp(None)
        .init();
    let _pool = databases::postgres_connection::start_connection().await;
    let json_web_token_environment =
        std::env::var("JSON_WEB_TOKEN_SECRET").expect("JSON_WEB_TOKEN must be set");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(AppState {
                postgres_client: _pool.clone(),
                json_web_token: json_web_token_environment.clone(),
            }))
            .service(hello) // <-- Adicione o serviço aqui
            .configure(services::users::services::config_users_routes) // <-- Adicione a configuração de rotas aqui)
            .configure(services::customer::services::config_customers_routes) // <-- Adicione a configuração de rotas aqui)
            .wrap(exceptions::errorHandleMiddleware::ErrorHandlerMiddleware)
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
