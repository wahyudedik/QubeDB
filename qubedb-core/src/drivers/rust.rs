//! Rust native driver for QubeDB
//! 
//! This module provides a native Rust driver for QubeDB
//! that can be used directly in Rust applications.

use crate::error::{QubeError, QubeResult};
use crate::drivers::DriverConfig;
use crate::query::QueryEngine;
use crate::storage::StorageEngine;
use std::collections::HashMap;

/// Rust native connection for QubeDB
pub struct RustConnection {
    config: DriverConfig,
    query_engine: QueryEngine,
    storage_engine: StorageEngine,
}

impl RustConnection {
    /// Create a new Rust connection
    pub fn new(config: DriverConfig) -> Self {
        RustConnection {
            config,
            query_engine: QueryEngine::new(),
            storage_engine: StorageEngine::new("./data").unwrap(),
        }
    }
    
    /// Execute a query
    pub async fn query(&self, sql: &str) -> QubeResult<RustResult> {
        let result = self.query_engine.execute_sql(sql).await?;
        
        Ok(RustResult {
            rows: result.rows,
            affected_rows: result.affected_rows,
        })
    }
}

/// Rust result
#[derive(Debug)]
pub struct RustResult {
    pub rows: Vec<HashMap<String, crate::types::Value>>,
    pub affected_rows: usize,
}
