//! Embedded QubeDB - SQLite-like embedded database
//! 
//! This module provides an embedded version of QubeDB that can be used
//! like SQLite - as a library embedded in applications.

use crate::error::{QubeError, QubeResult};
use crate::storage::StorageEngine;
use crate::query::QueryEngine;
use crate::types::{QueryResult, Row, Value};
use std::path::Path;
use std::collections::HashMap;

/// Embedded QubeDB instance
pub struct EmbeddedQubeDB {
    storage: StorageEngine,
    query_engine: QueryEngine,
    path: String,
}

impl EmbeddedQubeDB {
    /// Open or create an embedded QubeDB database
    pub fn open<P: AsRef<Path>>(path: P) -> QubeResult<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        let storage = StorageEngine::new(path.as_ref())?;
        let query_engine = QueryEngine::new();
        
        Ok(EmbeddedQubeDB {
            storage,
            query_engine,
            path: path_str,
        })
    }
    
    /// Execute a SQL query
    pub async fn execute(&self, sql: &str) -> QubeResult<QueryResult> {
        self.query_engine.execute_sql(sql).await
    }
    
    /// Insert a row into a table
    pub fn insert(&mut self, table: &str, row: Row) -> QubeResult<()> {
        // Generate a simple ID (in production, use proper ID generation)
        let id = format!("{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis());
            
        self.storage.put_row(table, &id, &row)
    }
    
    /// Get a row by ID
    pub fn get(&self, table: &str, id: &str) -> QubeResult<Option<Row>> {
        self.storage.get_row(table, id)
    }
    
    /// Update a row
    pub fn update(&mut self, table: &str, id: &str, row: Row) -> QubeResult<()> {
        self.storage.put_row(table, id, &row)
    }
    
    /// Delete a row
    pub fn delete(&mut self, table: &str, id: &str) -> QubeResult<()> {
        self.storage.delete_row(table, id)
    }
    
    /// Store a vector
    pub fn store_vector(&mut self, collection: &str, id: &str, vector: &[f32]) -> QubeResult<()> {
        self.storage.put_vector(collection, id, vector)
    }
    
    /// Get a vector
    pub fn get_vector(&self, collection: &str, id: &str) -> QubeResult<Option<Vec<f32>>> {
        self.storage.get_vector(collection, id)
    }
    
    /// Store a graph node
    pub fn store_node(&mut self, graph: &str, node_id: &str, properties: Row) -> QubeResult<()> {
        self.storage.put_graph_node(graph, node_id, &properties)
    }
    
    /// Store a graph edge
    pub fn store_edge(&mut self, graph: &str, from: &str, to: &str, properties: Row) -> QubeResult<()> {
        self.storage.put_graph_edge(graph, from, to, &properties)
    }
    
    /// Get database path
    pub fn path(&self) -> &str {
        &self.path
    }
}

/// Builder for creating embedded QubeDB instances
pub struct EmbeddedQubeDBBuilder {
    path: Option<String>,
}

impl EmbeddedQubeDBBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        EmbeddedQubeDBBuilder { path: None }
    }
    
    /// Set the database path
    pub fn path<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.path = Some(path.as_ref().to_string_lossy().to_string());
        self
    }
    
    /// Build the embedded database
    pub fn build(self) -> QubeResult<EmbeddedQubeDB> {
        let path = self.path.unwrap_or_else(|| "./qubedb_embedded".to_string());
        EmbeddedQubeDB::open(path)
    }
}

impl Default for EmbeddedQubeDBBuilder {
    fn default() -> Self {
        Self::new()
    }
}
