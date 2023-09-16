use axum::{extract::Path, response::IntoResponse};
use uuid::Uuid;

pub async fn read(Path(id): Path<Uuid>) -> impl IntoResponse {
    format!("GET {id}")
}

pub async fn remove(Path(id): Path<Uuid>) -> impl IntoResponse {
    format!("DELETE {id}")
}

pub async fn create() -> impl IntoResponse {
    "POST /person"
}

pub async fn update(Path(id): Path<Uuid>) -> impl IntoResponse {
    format!("PATCH {id}")
}
