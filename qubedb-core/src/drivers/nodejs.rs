//! Node.js driver for QubeDB
//! 
//! This module provides a Node.js native driver for QubeDB
//! that can be used with Express, NestJS, and other Node.js frameworks.

use crate::error::{QubeError, QubeResult};
use crate::drivers::DriverConfig;
use crate::query::QueryEngine;
use crate::storage::StorageEngine;
use std::collections::HashMap;

/// Node.js connection for QubeDB
pub struct NodeJSConnection {
    config: DriverConfig,
    query_engine: QueryEngine,
    storage_engine: StorageEngine,
}

impl NodeJSConnection {
    /// Create a new Node.js connection
    pub fn new(config: DriverConfig) -> Self {
        NodeJSConnection {
            config,
            query_engine: QueryEngine::new(),
            storage_engine: StorageEngine::new("./data").unwrap(),
        }
    }
    
    /// Execute a query
    pub async fn query(&self, sql: &str) -> QubeResult<NodeJSResult> {
        let result = self.query_engine.execute_sql(sql).await?;
        
        Ok(NodeJSResult {
            rows: result.rows,
            row_count: result.affected_rows,
        })
    }
}

/// Node.js result
#[derive(Debug)]
pub struct NodeJSResult {
    pub rows: Vec<HashMap<String, crate::types::Value>>,
    pub row_count: usize,
}
