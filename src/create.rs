use axum::{Extension, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TableDefinition {
    table_name: String,
    column_name: Vec<String>,
    column_type: Vec<String>,
}
use sqlx::{MySql, Pool};
pub async fn create(
    Extension(db): Extension<Pool<MySql>>,
    Json(data): Json<TableDefinition>,
) -> Json<String> {
    let mut create_query = format!("CREATE TABLE {} (", data.table_name);
    for (index, col_name) in data.column_name.iter().enumerate() {
        create_query.push_str(&format!("{} {}", col_name, data.column_type[index]));
        if index != data.column_name.len() - 1 && data.column_name.len() > 1 {
            create_query.push_str(&format!(","));
        }
    }
    create_query.push_str(&format!(");"));
    if let Err(e) = sqlx::query(&create_query).execute(&db).await {
        return Json(String::from(e.to_string()));
    };
    Json(String::from("Table Created!"))
}
