use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
mod app;
use app::build_app;
mod models;
mod routes;
mod state;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/postgres".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database_url)
        .expect("Failed to create database pool");

    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());
    let addr = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    let state = AppState::new(pool);
    let app = build_app(state);

    println!("Server started on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("server failed");
}
