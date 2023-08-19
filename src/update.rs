use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::{MySql, Pool};

#[derive(Deserialize)]
pub struct UpdateStruct {
    id: i32,
    title: Option<String>,
    body: Option<String>,
}
pub async fn update(
    Extension(db): Extension<Pool<MySql>>,
    Json(data): Json<UpdateStruct>,
) -> Json<String> {
    // let mut query: QueryBuilder<'_, MySql> = QueryBuilder::new("UPDATE posts SET");
    // let mut count = 0;

    let mut query: String = "".to_owned();
    let mut count = 0;
    if let Some(body) = data.body {
        if let Some(title) = data.title {
            query = format!(
                "UPDATE posts SET title = '{}', body = '{}' WHERE id = '{}'",
                title, body, data.id
            );
        } else {
            query = format!(
                "UPDATE posts SET body = '{}' WHERE id = '{}'",
                body, data.id
            );
        }
        count += 1
    } else if let Some(title) = data.title {
        if let Some(body) = data.body {
            query = format!(
                "UPDATE posts SET title = '{}', body = '{}' WHERE id = '{}'",
                title, body, data.id
            );
        } else {
            query = format!(
                "UPDATE posts SET title = '{}' WHERE id = '{}'",
                title, data.id
            );
        }
        count += 1;
    }
    if count == 0 {
        return Json(String::from("No updates made"));
    }

    if let Err(e) = sqlx::query(&query).execute(&db).await {
        return Json(String::from(e.to_string()));
    }
    Json(String::from("Data updated successfully"))
}
