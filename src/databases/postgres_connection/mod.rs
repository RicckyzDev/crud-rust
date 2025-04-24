use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn start_connection() -> Pool<Postgres> {
    let postgres_environment = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_environment)
        .await
        .expect("Failed to connect to postgres");

    let check_migrate = sqlx::migrate!("./src/databases/postgres_connection/migrations")
        .run(&pool)
        .await;

    match check_migrate {
        Ok(_) => println!("Migrations applied successfully"),
        Err(e) => println!("Error applying migrations: {}", e),
    }

    pool
}
