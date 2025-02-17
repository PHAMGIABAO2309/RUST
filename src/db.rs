use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok(); // Load biến môi trường từ .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    SqliteConnection::establish(&database_url)
        .expect("Error connecting to database")
}
