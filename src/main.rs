
use diesel::insert_into;
use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::types::Level;

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod types;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let connection = establish_connection();

    use crate::schema::logs::dsl::*;

    insert_into(logs).values(severity.eq(Level::Warn)).execute(&connection).expect("Error saving new log");
}
