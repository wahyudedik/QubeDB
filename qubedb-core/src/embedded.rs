//! Embedded QubeDB - SQLite-like embedded database
//! 
//! This module provides an embedded version of QubeDB that can be used
//! like SQLite - as a library embedded in applications.

use crate::error::QubeResult;
// use crate::storage::StorageEngine; // Temporarily disabled for real database implementation
use crate::query::QueryEngine;
use crate::types::{QueryResult, Row};
use crate::logging::{LogCategory, log_query, log_table, log_vector, log_graph, log_performance};
use std::path::Path;
use std::time::Instant;

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
        let start = Instant::now();
        
        // Log query start
        log_query(sql, true, 0).ok();
        
        let result = match self.query_engine.execute_sql(sql).await {
            Ok(result) => {
                let duration = start.elapsed();
                let duration_ms = duration.as_millis() as u64;
                
                // Log successful query
                log_query(sql, true, duration_ms).ok();
                
                // Log performance metrics
                log_performance("SQL Query", duration_ms, 0, 0.0).ok();
                
                Ok(result)
            },
            Err(e) => {
                let duration = start.elapsed();
                let duration_ms = duration.as_millis() as u64;
                
                // Log failed query
                log_query(sql, false, duration_ms).ok();
                
                // Log error
                crate::logging::log_error(LogCategory::Query, &format!("Query failed: {}", sql), &e, Some(format!("Duration: {}ms", duration_ms))).ok();
                
                Err(e)
            }
        };
        
        result
    }
    
    /// Insert a row into a table
    pub fn insert(&mut self, table: &str, row: Row) -> QubeResult<()> {
        let start = Instant::now();
        
        // Generate a simple ID (in production, use proper ID generation)
        let id = format!("{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis());
        
        let result = self.storage.put_row(table, &id, &row);
        
        let duration = start.elapsed();
        let duration_ms = duration.as_millis() as u64;
        
        match &result {
            Ok(_) => {
                // Log successful insert
                log_table("INSERT", table, true).ok();
                log_performance("Table Insert", duration_ms, 0, 0.0).ok();
            },
            Err(e) => {
                // Log failed insert
                log_table("INSERT", table, false).ok();
                crate::logging::log_error(LogCategory::Table, &format!("Insert failed for table: {}", table), e, Some(format!("Duration: {}ms", duration_ms))).ok();
            }
        }
        
        result
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
        let start = Instant::now();
        
        let result = self.storage.put_vector(collection, id, vector);
        
        let duration = start.elapsed();
        let duration_ms = duration.as_millis() as u64;
        
        match &result {
            Ok(_) => {
                // Log successful vector store
                log_vector("STORE", collection, true, duration_ms).ok();
                log_performance("Vector Store", duration_ms, 0, 0.0).ok();
            },
            Err(e) => {
                // Log failed vector store
                log_vector("STORE", collection, false, duration_ms).ok();
                crate::logging::log_error(LogCategory::Vector, &format!("Vector store failed for collection: {}", collection), e, Some(format!("Duration: {}ms", duration_ms))).ok();
            }
        }
        
        result
    }
    
    /// Get a vector
    pub fn get_vector(&self, collection: &str, id: &str) -> QubeResult<Option<Vec<f32>>> {
        self.storage.get_vector(collection, id)
    }
    
    /// Store a graph node
    pub fn store_node(&mut self, graph: &str, node_id: &str, properties: Row) -> QubeResult<()> {
        let start = Instant::now();
        
        let result = self.storage.put_graph_node(graph, node_id, &properties);
        
        let duration = start.elapsed();
        let duration_ms = duration.as_millis() as u64;
        
        match &result {
            Ok(_) => {
                // Log successful graph node store
                log_graph("STORE_NODE", graph, true).ok();
                log_performance("Graph Node Store", duration_ms, 0, 0.0).ok();
            },
            Err(e) => {
                // Log failed graph node store
                log_graph("STORE_NODE", graph, false).ok();
                crate::logging::log_error(LogCategory::Graph, &format!("Graph node store failed for graph: {}", graph), e, Some(format!("Duration: {}ms", duration_ms))).ok();
            }
        }
        
        result
    }
    
    /// Store a graph edge
    pub fn store_edge(&mut self, graph: &str, from: &str, to: &str, properties: Row) -> QubeResult<()> {
        let start = Instant::now();
        
        let result = self.storage.put_graph_edge(graph, from, to, &properties);
        
        let duration = start.elapsed();
        let duration_ms = duration.as_millis() as u64;
        
        match &result {
            Ok(_) => {
                // Log successful graph edge store
                log_graph("STORE_EDGE", graph, true).ok();
                log_performance("Graph Edge Store", duration_ms, 0, 0.0).ok();
            },
            Err(e) => {
                // Log failed graph edge store
                log_graph("STORE_EDGE", graph, false).ok();
                crate::logging::log_error(LogCategory::Graph, &format!("Graph edge store failed for graph: {}", graph), e, Some(format!("Duration: {}ms", duration_ms))).ok();
            }
        }
        
        result
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
