//! API module for QubeDB
//! Provides REST and GraphQL endpoints

pub mod rest;
pub mod graphql;
pub mod middleware;

use crate::types::QueryResult;
use crate::error::QubeResult;
use std::collections::HashMap;

/// API server configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub enable_cors: bool,
    pub enable_auth: bool,
    pub max_request_size: usize,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_cors: true,
            enable_auth: false,
            max_request_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// API server trait
pub trait ApiServer {
    async fn start(&self) -> QubeResult<()>;
    async fn stop(&self) -> QubeResult<()>;
    async fn health_check(&self) -> QubeResult<bool>;
}

/// Request context for API handlers
#[derive(Debug, Clone)]
pub struct RequestContext {
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
    pub request_id: String,
    pub timestamp: u64,
}

/// API response wrapper
#[derive(Debug, Clone, serde::Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: u64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}
