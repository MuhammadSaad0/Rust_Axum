use axum::Extension;
use axum::Json;
use serde::Deserialize;
use sqlx::{MySql, Pool};

#[derive(Deserialize)]
pub struct DeleteStruct {
    id: i32,
}
pub async fn delete(
    Extension(db): Extension<Pool<MySql>>,
    Json(data): Json<DeleteStruct>,
) -> Json<String> {
    let query = format!("DELETE FROM posts WHERE ID = '{}'", data.id);

    if let Err(e) = sqlx::query(&query).execute(&db).await {
        return Json(String::from(e.to_string()));
    }

    Json(String::from("Data successfully deleted"))
}
