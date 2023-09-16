pub mod models;
pub mod schema;

use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenvy::dotenv;
use models::Person;
use std::env;
use uuid::Uuid;

use crate::models::NewPerson;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_person(conn: &mut SqliteConnection, name: &str, age: u8) {
    use crate::schema::persons;

    let new_person = NewPerson {
        id: Uuid::new_v4(),
        name,
        age,
        favourite_colour: models::Colour::Black,
    };

    diesel::insert_into(persons::table)
        .values(new_person)
        .returning(Person::as_returning())
        .get_result(conn)
        .expect("Error saving new person");
}
