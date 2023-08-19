use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::{MySql, Pool};

#[derive(Deserialize)]
pub struct InsertStruct {
    title: String,
    body: String,
}

pub async fn insert(
    Extension(db): Extension<Pool<MySql>>,
    Json(data): Json<InsertStruct>,
) -> Json<String> {
    let query = format!(
        "INSERT INTO posts (title, body) VALUES ('{}', '{}')",
        data.title, data.body
    );

    if let Err(e) = sqlx::query(&query).execute(&db).await {
        return Json(String::from(e.to_string()));
    }

    Json(String::from("Data successfully inserted"))
}
