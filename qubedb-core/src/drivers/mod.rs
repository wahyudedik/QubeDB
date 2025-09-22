//! Database drivers for QubeDB
//!
//! This module provides drivers for popular ORMs and frameworks:
//! - Laravel (PHP) - PDO driver
//! - Django (Python) - Django ORM backend
//! - Spring Boot (Java) - JDBC driver
//! - Node.js - Native driver
//! - Go - database/sql driver
//! - Rust - Native driver

pub mod django;
pub mod go;
pub mod jdbc;
pub mod nodejs;
pub mod pdo;
pub mod rust;

/// Driver configuration
#[derive(Debug, Clone)]
pub struct DriverConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl: bool,
    pub timeout: u64,
}

impl Default for DriverConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            database: "qubedb".to_string(),
            username: "admin".to_string(),
            password: "".to_string(),
            ssl: false,
            timeout: 30,
        }
    }
}
