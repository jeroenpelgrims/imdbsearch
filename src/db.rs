use serde::Deserialize;
use sqlx::{sqlite::SqlitePool, Execute, QueryBuilder, Sqlite};

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

#[derive(Clone, Deserialize, Debug, serde::Serialize)]
pub struct GetTitlesParams {
    pub page: Option<i32>,
    pub title: Option<String>,
    pub min_runtime: Option<i32>,
    pub max_runtime: Option<i32>,
    pub min_score: Option<f32>,
    pub max_score: Option<f32>,
    pub min_votes: Option<i32>,
    pub max_votes: Option<i32>,
    pub min_premiered: Option<i32>,
    pub max_premiered: Option<i32>,
    pub genres: Option<Vec<String>>,
}

impl GetTitlesParams {
    pub fn next_page(&self) -> String {
        let next_params = GetTitlesParams {
            page: Some(self.page.unwrap_or(1) + 1),
            ..self.clone()
        };
        serde_qs::to_string(&next_params).unwrap()
    }

    pub fn prev_page(&self) -> Option<String> {
        let page = self.page.unwrap_or(1);

        if (page - 1) <= 0 {
            return None;
        }

        let next_params = GetTitlesParams {
            page: Some(self.page.unwrap_or(1) - 1),
            ..self.clone()
        };
        Some(serde_qs::to_string(&next_params).unwrap())
    }
}

pub async fn get_titles(pool: SqlitePool, params: GetTitlesParams) -> Vec<Title> {
    const PAGE_SIZE: i32 = 50;
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

    if let Some(min_runtime) = params.min_runtime {
        qb.push(" AND runtime_minutes >= ").push_bind(min_runtime);
    }

    if let Some(max_runtime) = params.max_runtime {
        qb.push(" AND runtime_minutes <= ").push_bind(max_runtime);
    }

    if let Some(min_votes) = params.min_votes {
        qb.push(" AND votes >= ").push_bind(min_votes);
    }

    if let Some(max_votes) = params.max_votes {
        qb.push(" AND votes <= ").push_bind(max_votes);
    }

    if let Some(min_score) = params.min_score {
        qb.push(" AND rating >= ").push_bind(min_score);
    }

    if let Some(max_score) = params.max_score {
        qb.push(" AND rating <= ").push_bind(max_score);
    }

    if let Some(min_premiered) = params.min_premiered {
        qb.push(" AND premiered >= ").push_bind(min_premiered);
    }

    if let Some(max_premiered) = params.max_premiered {
        qb.push(" AND premiered <= ").push_bind(max_premiered);
    }

    if let Some(genres) = params.genres {
        qb.push(" AND (");
        for (i, genre) in genres.iter().enumerate() {
            if i > 0 {
                qb.push(" OR ");
            }
            qb.push(" genres LIKE ").push_bind(format!("%{}%", genre));
        }
        qb.push(" ) ");
    }

    let page = (params.page.unwrap_or(1) - 1) * PAGE_SIZE;
    qb.push(" LIMIT ").push_bind(PAGE_SIZE);
    qb.push(" OFFSET ").push_bind(page);

    let query = qb.build_query_as();
    query.fetch_all(&pool).await.unwrap()
}
