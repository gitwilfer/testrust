// src/bin/test_connections.rs
use anyb::infrastructure::persistence::database;

fn main() {
    println!("Testing database connection...");
    
    match database::get_default_connection() {
        Ok(_) => println!("✅ Successfully connected to the database!"),
        Err(e) => println!("❌ Failed to connect to the database: {}", e),
    }
}