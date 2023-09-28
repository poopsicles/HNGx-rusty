use axum::{extract::Path, response::IntoResponse, Json};
use diesel::SqliteConnection;
use serde::Deserialize;
use uuid::Uuid;

pub async fn read(Path(id): Path<Uuid>) -> impl IntoResponse {
    format!("GET {id}")
}

pub async fn remove(Path(id): Path<Uuid>) -> impl IntoResponse {
    format!("DELETE {id}")
}

pub async fn create(payload: Json<CreatePerson>) -> impl IntoResponse {
    format!("Attempting to create someone with {}, {}, and {:?}", payload.name, payload.age, payload.favourite_colour)
}

pub async fn update(Path(id): Path<Uuid>) -> impl IntoResponse {
    format!("PATCH {id}")
}

pub struct ErrorList {
    pub errors: Vec<String>
}

#[derive(Deserialize)]
pub struct CreatePerson {
    pub name: String,
    pub age: u8,
    pub favourite_colour: String
}

impl CreatePerson {
    pub fn validate(&self, conn: &mut SqliteConnection) -> Result<(), ErrorList> {
        
    }
}