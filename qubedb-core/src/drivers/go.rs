//! Go driver for QubeDB
//! 
//! This module provides a Go database/sql driver for QubeDB
//! that can be used with Go applications.

use crate::error::{QubeError, QubeResult};
use crate::drivers::DriverConfig;
use crate::query::QueryEngine;
use crate::storage::StorageEngine;
use std::collections::HashMap;

/// Go connection for QubeDB
pub struct GoConnection {
    config: DriverConfig,
    query_engine: QueryEngine,
    storage_engine: StorageEngine,
}

impl GoConnection {
    /// Create a new Go connection
    pub fn new(config: DriverConfig) -> Self {
        GoConnection {
            config,
            query_engine: QueryEngine::new(),
            storage_engine: StorageEngine::new("./data").unwrap(),
        }
    }
    
    /// Execute a query
    pub async fn query(&self, sql: &str) -> QubeResult<GoResult> {
        let result = self.query_engine.execute_sql(sql).await?;
        
        Ok(GoResult {
            rows: result.rows,
            affected_rows: result.affected_rows,
        })
    }
}

/// Go result
#[derive(Debug)]
pub struct GoResult {
    pub rows: Vec<HashMap<String, crate::types::Value>>,
    pub affected_rows: usize,
}
