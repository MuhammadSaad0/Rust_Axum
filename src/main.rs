pub mod create;
pub mod delete;
pub mod get;
pub mod insert;
pub mod update;
use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::get,
    routing::post,
    Extension, Router,
};
use create::create;
use delete::delete;
use get::{getposts, gettableinfo, gettables};
use insert::insert;
use sqlx::mysql::MySqlPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use update::update;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let pool = MySqlPoolOptions::new()
        .connect("mysql://root:36903690@localhost/axum")
        .await
        .unwrap();

    let app = Router::new()
        .route("/insert", post(insert))
        .route("/delete", post(delete))
        .route("/update", post(update))
        .route("/getposts", get(getposts))
        .route("/gettables", get(gettables))
        .route("/gettableinfo/:table", get(gettableinfo))
        .route("/create", post(create))
        .layer(cors)
        .layer(Extension(pool));

    axum::Server::bind(&"0.0.0.0:3690".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
