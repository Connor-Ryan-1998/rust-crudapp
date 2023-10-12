mod handlers;
use axum::http;
use axum::routing::{get, post, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: String = format!("0.0.0.0:{}", port);

    let database_url = env::var("DATBASE_URL").expect("missing DATABASE_URL env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app: Router = Router::new()
        .route("/", get(health))
        .route("/quotes", post(handlers::create_quote))
        .with_state(pool);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health() -> http::StatusCode {
    http::StatusCode::OK
}
