//! QubeDB Server Binary
//! 
//! This is the main server executable for QubeDB

use qubedb_core::{QubeDB, QubeError};
use std::env;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting QubeDB Server...");
    
    // Get database path from environment or use default
    let db_path = env::var("QUBEDB_PATH").unwrap_or_else(|_| "./data".to_string());
    let port = env::var("QUBEDB_PORT").unwrap_or_else(|_| "8080".to_string());
    
    info!("Database path: {}", db_path);
    info!("Server port: {}", port);
    
    // Create QubeDB instance
    let qubedb = match QubeDB::new() {
        Ok(db) => {
            info!("QubeDB initialized successfully");
            db
        }
        Err(e) => {
            error!("Failed to initialize QubeDB: {}", e);
            return Err(e.into());
        }
    };
    
    // Start the database
    if let Err(e) = qubedb.start().await {
        error!("Failed to start QubeDB: {}", e);
        return Err(e.into());
    }
    
    info!("QubeDB server is running!");
    info!("Press Ctrl+C to stop the server");
    
    // Keep the server running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down QubeDB server...");
    
    Ok(())
}
