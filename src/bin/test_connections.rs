// src/bin/test_connections.rs
use anyb::infrastructure::persistence::establish_connection;

fn main() {
    println!("Testing database connection...");
    
    match establish_connection().get() {
        Ok(_) => println!("✅ Successfully connected to the database!"),
        Err(e) => println!("❌ Failed to connect to the database: {}", e),
    }
}
