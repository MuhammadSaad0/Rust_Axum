use std::any::Any;

use axum::{extract::Path, Extension, Json};
use serde::Serialize;
use sqlx::{Column, MySql, Pool, Row};

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

#[derive(Debug, Serialize)]
pub struct TablesInfoStruct {
    table_name: String,
}
pub async fn gettables(Extension(db): Extension<Pool<MySql>>) -> Json<Vec<TablesInfoStruct>> {
    let query = format!("SHOW tables");
    let result = sqlx::query(&query).fetch_all(&db).await.unwrap();
    let to_ret: Vec<TablesInfoStruct> = result
        .iter()
        .map(|row| TablesInfoStruct {
            table_name: row.get("Tables_in_axum"),
        })
        .collect();
    Json(to_ret)
}

#[derive(Debug, Serialize)]
pub struct TableDescStruct {
    field: String,
    datatype: String,
}

struct TestStruct {
    value: Box<dyn Any>,
}

pub async fn gettableinfo(
    Extension(db): Extension<Pool<MySql>>,
    Path(table): Path<String>,
) -> Json<Vec<TableDescStruct>> {
    let query = format!("DESCRIBE {}", table);
    let result = sqlx::query(&query).fetch_all(&db).await.unwrap();
    let to_ret: Vec<TableDescStruct> = result
        .iter()
        .map(|row| TableDescStruct {
            field: row.get("Field"),
            datatype: row.get("Type"),
        })
        .collect();

    /* Problematic
    // let query2 = format!("SELECT * FROM {}", table);
    // let result2 = sqlx::query(&query2).fetch_all(&db).await.unwrap();
    // Json(result2);

    // let query2 = format!("SELECT * FROM {}", table);
    // let result2 = sqlx::query(&query2).fetch_all(&db).await.unwrap();
    // for row in result2.iter() {
    //     for col in row.columns() {
    //         let value_any: &dyn Any = row.try_get(col.name()).unwrap().as_ref().as_any();
    //     }
    // }
    */
    Json(to_ret)
}
