//! Storage engine for QubeDB
//! 
//! This module provides the storage layer that can handle:
//! - Relational data (B-Tree indexes)
//! - Document data (JSON storage)
//! - Vector data (for AI/ML)
//! - Graph data (nodes and edges)

use crate::error::{QubeError, QubeResult};
use crate::types::{Row, Value, Table};
use serde_json;
use std::path::Path;
use std::collections::HashMap;

/// Storage engine that handles different data models
pub struct StorageEngine {
    data: HashMap<String, Vec<u8>>,
    path: String,
}

impl StorageEngine {
    /// Create a new storage engine
    pub fn new<P: AsRef<Path>>(path: P) -> QubeResult<Self> {
        // Create directory if it doesn't exist
        std::fs::create_dir_all(path.as_ref())
            .map_err(|e| QubeError::Storage(format!("Failed to create directory: {}", e)))?;
            
        Ok(StorageEngine {
            data: HashMap::new(),
            path: path.as_ref().to_string_lossy().to_string(),
        })
    }
    
    /// Store a row in a table
    pub fn put_row(&mut self, table: &str, key: &str, row: &Row) -> QubeResult<()> {
        let serialized = serde_json::to_vec(row)
            .map_err(|e| QubeError::Serialization(format!("Failed to serialize row: {}", e)))?;
            
        let db_key = format!("{}:{}", table, key);
        self.data.insert(db_key, serialized);
        Ok(())
    }
    
    /// Get a row from a table
    pub fn get_row(&self, table: &str, key: &str) -> QubeResult<Option<Row>> {
        let db_key = format!("{}:{}", table, key);
        match self.data.get(&db_key) {
            Some(data) => {
                let row: Row = serde_json::from_slice(data)
                    .map_err(|e| QubeError::Serialization(format!("Failed to deserialize row: {}", e)))?;
                Ok(Some(row))
            }
            None => Ok(None),
        }
    }
    
    /// Delete a row from a table
    pub fn delete_row(&mut self, table: &str, key: &str) -> QubeResult<()> {
        let db_key = format!("{}:{}", table, key);
        self.data.remove(&db_key);
        Ok(())
    }
    
    /// Store vector data for AI/ML
    pub fn put_vector(&mut self, collection: &str, id: &str, vector: &[f32]) -> QubeResult<()> {
        let serialized = bincode::serialize(vector)
            .map_err(|e| QubeError::Serialization(format!("Failed to serialize vector: {}", e)))?;
            
        let db_key = format!("vector:{}:{}", collection, id);
        self.data.insert(db_key, serialized);
        Ok(())
    }
    
    /// Get vector data
    pub fn get_vector(&self, collection: &str, id: &str) -> QubeResult<Option<Vec<f32>>> {
        let db_key = format!("vector:{}:{}", collection, id);
        match self.data.get(&db_key) {
            Some(data) => {
                let vector: Vec<f32> = bincode::deserialize(data)
                    .map_err(|e| QubeError::Serialization(format!("Failed to deserialize vector: {}", e)))?;
                Ok(Some(vector))
            }
            None => Ok(None),
        }
    }
    
    /// Store graph node
    pub fn put_graph_node(&mut self, graph: &str, node_id: &str, properties: &Row) -> QubeResult<()> {
        let serialized = serde_json::to_vec(properties)
            .map_err(|e| QubeError::Serialization(format!("Failed to serialize node: {}", e)))?;
            
        let db_key = format!("graph:{}:node:{}", graph, node_id);
        self.data.insert(db_key, serialized);
        Ok(())
    }
    
    /// Store graph edge
    pub fn put_graph_edge(&mut self, graph: &str, from: &str, to: &str, properties: &Row) -> QubeResult<()> {
        let serialized = serde_json::to_vec(properties)
            .map_err(|e| QubeError::Serialization(format!("Failed to serialize edge: {}", e)))?;
            
        let db_key = format!("graph:{}:edge:{}:{}", graph, from, to);
        self.data.insert(db_key, serialized);
        Ok(())
    }
}
