use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use hngx_duo::handlers::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api", post(create))
        .route("/api/:id", get(read))
        .route("/api/:id", patch(update))
        .route("/api/:id", delete(remove));

    println!("Listening on port 80");

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

//     create_person(conn, "fum", 10);

//     let res = persons
//         .limit(10)
//         .select(Person::as_select())
//         .load(conn)
//         .expect("Error loading persons");

//     println!("displaying {} people", res.len());

//     for person in res {
//         println!("{}", person.name);
//     }
}
