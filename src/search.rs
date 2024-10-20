use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db;

pub fn search_router() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/", get(get_index))
        .route("/", post(post_index))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    titles: Vec<db::Title>,
}

async fn get_index(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let params = db::GetTitlesParams {
        ..Default::default()
    };
    let titles = db::get_titles(pool, params).await;
    let html = IndexTemplate { titles }.render().unwrap();
    (StatusCode::OK, Html(html).into_response())
}

#[derive(Template)]
#[template(path = "titles-table.html")]
struct TitlesTableTemplate {
    titles: Vec<db::Title>,
}

async fn post_index(
    State(pool): State<SqlitePool>,
    Form(form_data): Form<db::GetTitlesParams>,
) -> impl IntoResponse {
    let titles = db::get_titles(pool, form_data).await;
    let html = TitlesTableTemplate { titles }.render().unwrap();
    (StatusCode::OK, Html(html).into_response())
}
