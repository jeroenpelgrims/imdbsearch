use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db;

pub fn search_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/", get(index))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    titles: Vec<db::Title>,
}

async fn index(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let params = db::GetTitlesParams {
        ..Default::default()
    };
    let titles = db::get_titles(pool, params).await;
    let html = IndexTemplate { titles }.render().unwrap();
    (StatusCode::OK, Html(html).into_response())
}
