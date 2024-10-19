use axum::Router;
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
mod db;
mod search;

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
        .nest("/", search::search_router())
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(pool);

    if cfg!(debug_assertions) {
        println!("Running in debug mode, enabling LiveReload");
        app = app.layer(LiveReloadLayer::new());
    }
    app = app.layer(CompressionLayer::new());

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("Listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}

// async fn index(State(pool): State<SqlitePool>) -> Result<String, (StatusCode, String)> {
//     let params = GetTitlesParams {
//         page: Some(0),
//         ..Default::default()
//     };
//     let titles = db::get_titles(pool, params).await;
//     let s = titles
//         .iter()
//         .map(|x| format!("{:?}", x))
//         .collect::<Vec<String>>()
//         .join("\n");
//     Ok(s)
// }
