use axum::{Extension, Json};
use serde::Serialize;
use sqlx::{MySql, Pool, Row};

#[derive(Debug, Serialize)]
pub struct ReturnStruct {
    id: i32,
    title: String,
    body: String,
}

pub async fn getposts(Extension(db): Extension<Pool<MySql>>) -> Json<Vec<ReturnStruct>> {
    let query = format!("SELECT * FROM posts");
    let rows = sqlx::query(&query).fetch_all(&db).await.unwrap();
    let result: Vec<ReturnStruct> = rows
        .iter()
        .map(|row| ReturnStruct {
            id: row.get("id"),
            title: row.get("title"),
            body: row.get("body"),
        })
        .collect();
    Json(result)
}
