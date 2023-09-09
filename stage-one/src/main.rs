mod data;

use axum::{extract::Query, routing::get, Json, Router};
use data::Data;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiParams {
    slack_name: String,
    track: String,
}

async fn api(params: Query<ApiParams>) -> Json<Data> {
    Json(Data::from(params.0))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api", get(api));

    println!("Listening on port 80");

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
