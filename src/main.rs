use axum::{extract::State, http::StatusCode, routing::get, Router};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();
    println!("Connecting to database: {}", db_connection_str);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let mut app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(pool)
        .layer(CompressionLayer::new());

    if cfg!(debug_assertions) {
        app = app.layer(LiveReloadLayer::new());
    }

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

// async fn index(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
//     let messages = sqlx::query("select * from message")
//         .fetch_one(&pool)
//         .await
//         .map_err(internal_error);
//     println!("{:?}", messages);
//     sqlx::query_scalar("select 'hello world from pg'")
//         .fetch_one(&pool)
//         .await
//         .map_err(internal_error)
//     // (StatusCode::OK, "OK".into_response())
// }

// async fn db_test(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
//     let messages = sqlx::query("select * from message")
//         .fetch_one(&pool)
//         .await
//         .map_err(internal_error);
//     println!("{:?}", messages);
//     sqlx::query_scalar("select 'hello world from pg'")
//         .fetch_one(&pool)
//         .await
//         .map_err(internal_error)
//     // (StatusCode::OK, "OK".into_response())
// }

// fn internal_error<E>(err: E) -> (StatusCode, String)
// where
//     E: std::error::Error,
// {
//     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
// }
