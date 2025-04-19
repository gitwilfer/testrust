// src/bin/test_connections.rs
use anyb::Infrastructure::Persistence::connection_pools;

fn main() {
    println!("Testing database connection...");
    
    match connection_pools::get_default_connection() {
        Ok(_) => println!("✅ Successfully connected to the database!"),
        Err(e) => println!("❌ Failed to connect to the database: {}", e),
    }
}