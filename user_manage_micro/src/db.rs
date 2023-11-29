// Database Connection Setup
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
