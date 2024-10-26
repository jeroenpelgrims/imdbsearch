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
    next_page: i32,
    form_data: &'a db::GetTitlesParams,
}

#[derive(Template)]
#[template(path = "title-row.html")]
struct TitleRowTemplate {
    title: db::Title,
    next_page: i32,
    is_last: bool,
}

async fn get_index(
    headers: HeaderMap,
    State(pool): State<SqlitePool>,
    Form(form_data): Form<db::GetTitlesParams>,
) -> impl IntoResponse {
    let page = form_data.page.unwrap_or(1);
    let titles = db::get_titles(pool, form_data.clone()).await;
    let html = if headers.contains_key("hx-request") {
        let titles_count = titles.len();
        let templates: Vec<String> = titles
            .into_iter()
            .enumerate()
            .map(|(i, title)| TitleRowTemplate {
                title,
                next_page: page + 1,
                is_last: i == titles_count - 1,
            })
            .map(|template| template.render().unwrap())
            .collect();
        templates.join("\n")
    } else {
        IndexTemplate {
            titles,
            form_data: &form_data,
            next_page: page + 1,
        }
        .render()
        .unwrap()
    };
    (StatusCode::OK, Html(html).into_response())
}
