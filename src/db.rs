use super::extractor::empty_string_as_none;
use serde::Deserialize;
use sqlx::{sqlite::SqlitePool, QueryBuilder, Sqlite};

#[derive(sqlx::FromRow, Debug, PartialEq)]
pub struct Title {
    pub title_id: String,
    pub primary_title: String,
    pub original_title: String,
    pub premiered: i32,
    pub runtime: i32,
    pub rating: f32,
    pub votes: i32,
}

#[derive(Clone, Default, Deserialize, Debug)]
pub struct GetTitlesParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub page: Option<i32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub title: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub max_runtime: Option<i32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub min_score: Option<f32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub max_score: Option<f32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub min_votes: Option<i32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub max_votes: Option<i32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub min_premiered: Option<i32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub max_premiered: Option<i32>,
}

pub async fn get_titles(pool: SqlitePool, params: GetTitlesParams) -> Vec<Title> {
    const PAGE_SIZE: i32 = 10;
    let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT
                t.title_id,
                primary_title,
                original_title,
                premiered,
                runtime_minutes as runtime,
                CAST(r.rating AS REAL) AS rating,
                r.votes
            FROM titles t
            INNER JOIN ratings r ON t.title_id = r.title_id
            WHERE type = 'movie' ",
    );

    if let Some(title) = params.title {
        qb.push(" AND primary_title LIKE ")
            .push_bind(format!("%{}%", title))
            .push(" COLLATE NOCASE ");
    }

    if let Some(max_runtime) = params.max_runtime {
        qb.push(" AND runtime_minutes < ").push_bind(max_runtime);
    }

    if let Some(min_votes) = params.min_votes {
        qb.push(" AND votes > ").push_bind(min_votes);
    }

    if let Some(max_votes) = params.max_votes {
        qb.push(" AND votes < ").push_bind(max_votes);
    }

    if let Some(min_score) = params.min_score {
        qb.push(" AND rating > ").push_bind(min_score);
    }

    if let Some(max_score) = params.max_score {
        qb.push(" AND rating < ").push_bind(max_score);
    }

    if let Some(min_premiered) = params.min_premiered {
        qb.push(" AND premiered > ").push_bind(min_premiered);
    }

    if let Some(max_premiered) = params.max_premiered {
        qb.push(" AND premiered < ").push_bind(max_premiered);
    }

    qb.push(" LIMIT ").push_bind(PAGE_SIZE);
    qb.push(" OFFSET ")
        .push_bind(params.page.unwrap_or(0) * PAGE_SIZE);

    qb.build_query_as().fetch_all(&pool).await.unwrap()
}
