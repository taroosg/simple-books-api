use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::get, Json, Router,
};
use dotenv::dotenv;
use serde::Serialize;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Book {
    id: i64,
    title: String,
    isbn: String,
}

async fn get_books(Extension(pool): Extension<sqlx::Pool<sqlx::Sqlite>>) -> impl IntoResponse {
    let books = sqlx::query_as!(Book, "SELECT id, title, isbn FROM books")
        .fetch_all(&pool)
        .await;

    match books {
        Ok(book_list) => (StatusCode::OK, Json(book_list)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let app = Router::new()
        .route("/books", get(get_books))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
