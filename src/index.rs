use askama::Template;
use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use sqlx::{Pool, Sqlite, SqlitePool};
use tracing_subscriber::field::debug;

use crate::db;
use crate::extractor::filters;

pub fn index_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/", get(get_index))
    // .route("/", post(post_index))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    titles: Vec<db::Title>,
    form_data: &'a db::GetTitlesParams,
}

#[derive(Template)]
#[template(path = "titles-table.html")]
struct TitlesTableTemplate {
    titles: Vec<db::Title>,
}

async fn get_index(
    headers: HeaderMap,
    State(pool): State<SqlitePool>,
    Form(form_data): Form<db::GetTitlesParams>,
) -> impl IntoResponse {
    let titles = db::get_titles(pool, form_data.clone()).await;
    let html = if headers.contains_key("hx-request") {
        TitlesTableTemplate { titles }.render().unwrap()
    } else {
        IndexTemplate {
            titles,
            form_data: &form_data,
        }
        .render()
        .unwrap()
    };
    (StatusCode::OK, Html(html).into_response())
}
