use askama::Template;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_extra::extract::Form;
use indexmap::IndexMap;
use serde_qs;
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::db;
use crate::extractor::filters;

const GENRES: [&str; 27] = [
    "Action",
    "Adult",
    "Adventure",
    "Animation",
    "Biography",
    "Comedy",
    "Crime",
    "Documentary",
    "Drama",
    "Family",
    "Fantasy",
    "Game-Show",
    "History",
    "Horror",
    "Music",
    "Musical",
    "Mystery",
    "News",
    "Reality-TV",
    "Romance",
    "Sci-Fi",
    "Short",
    "Sport",
    "Talk-Show",
    "Thriller",
    "War",
    "Western",
];

pub fn index_router() -> Router<Pool<Sqlite>> {
    Router::new().route("/", get(get_index))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    titles: Vec<db::Title>,
    next_page: &'a str,
    form_data: &'a db::GetTitlesParams,
    genres: IndexMap<String, bool>,
}

#[derive(Template)]
#[template(path = "title-row.html")]
struct TitleRowTemplate<'a> {
    title: db::Title,
    next_page: &'a str,
    is_last: bool,
}

async fn get_index(
    headers: HeaderMap,
    State(pool): State<SqlitePool>,
    Form(form_data): Form<db::GetTitlesParams>,
) -> impl IntoResponse {
    let page = form_data.page.unwrap_or(1);
    let next_page_qs = serde_qs::to_string(&db::GetTitlesParams {
        page: Some(page + 1),
        ..form_data.clone()
    })
    .unwrap();

    let titles = db::get_titles(pool, form_data.clone()).await;
    let html = if headers.contains_key("hx-request") {
        let titles_count = titles.len();
        let templates: Vec<String> = titles
            .into_iter()
            .enumerate()
            .map(|(i, title)| TitleRowTemplate {
                title,
                next_page: &next_page_qs,
                is_last: i == titles_count - 1,
            })
            .map(|template| template.render().unwrap())
            .collect();
        templates.join("\n")
    } else {
        let genres_selected = GENRES
            .iter()
            .map(|genre| {
                let genre_owned = genre.to_string();
                let selected = form_data
                    .genres
                    .as_ref()
                    .is_some_and(|genres| genres.contains(&genre_owned))
                    .clone();
                (genre_owned, selected)
            })
            .collect::<IndexMap<String, bool>>();

        IndexTemplate {
            titles,
            form_data: &form_data,
            next_page: &next_page_qs,
            genres: genres_selected,
        }
        .render()
        .unwrap()
    };
    (StatusCode::OK, Html(html).into_response())
}
