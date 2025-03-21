use diesel::prelude::*;
use dotenvy::dotenv;
mod db;
mod config;
use config::Config;

fn main() {
    // Load .env file
    dotenv().ok();

    // Access environment variables
    let config: Config = Config::new().expect("Failed to load configuration");

    // Pass the database_url to establish_connection
    let connection = db::establish_connection(&config.database_url);

    // Rest of your code...
}