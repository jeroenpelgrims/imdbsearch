use serde::Deserialize;
use sqlx::{query, query_as, sqlite::SqlitePool, Execute, QueryBuilder, Sqlite};

use crate::db;

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

#[derive(Default, Deserialize)]
pub struct GetTitlesParams {
    pub page: Option<i32>,
    pub title: Option<String>,
    pub max_runtime: Option<i32>,
    pub min_score: Option<f32>,
    pub min_votes: Option<i32>,
    pub min_premiered: Option<i32>,
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
            WHERE 1 = 1 ",
    );

    if let Some(title) = params.title {
        qb.push("AND (primary_title LIKE ")
            .push_bind(format!("%{}%", title))
            .push("OR original_title LIKE ")
            .push_bind(format!("%{}%", title))
            .push(") ");
    }

    if let Some(max_runtime) = params.max_runtime {
        qb.push(" AND runtime_minutes < ").push_bind(max_runtime);
    }

    if let Some(min_votes) = params.min_votes {
        qb.push(" AND votes > ").push_bind(min_votes);
    }

    if let Some(min_score) = params.min_score {
        qb.push(" AND rating > ").push_bind(min_score);
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

struct Search {
    id: i64,
    username: Option<String>,
    min_age: Option<i8>,
    max_age: Option<i8>,
}

fn search_query(search: Search) -> String {
    let mut query = QueryBuilder::new("SELECT * from users where id = ");
    query.push_bind(search.id);

    if let Some(username) = search.username {
        query.push(" AND username = ");
        query.push_bind(username);
    }

    if let Some(min_age) = search.min_age {
        query.push(" AND age > ");
        query.push_bind(min_age);
    }

    if let Some(max_age) = search.max_age {
        query.push(" AND age < ");
        query.push_bind(max_age);
    }

    query.build().sql().into()
}

pub fn main() {
    dbg!(search_query(Search {
        id: 12,
        username: None,
        min_age: None,
        max_age: None,
    })); // "SELECT * from users where id = $1"
    dbg!(search_query(Search {
        id: 12,
        username: Some("Bob".into()),
        min_age: None,
        max_age: None,
    })); // "SELECT * from users where id = $1 AND username = $2"
    dbg!(search_query(Search {
        id: 12,
        username: Some("Bob".into()),
        min_age: Some(10),
        max_age: Some(70),
    })); // "SELECT * from users where id = $1 AND username = $2 AND age > $3 AND age < $4"
}
