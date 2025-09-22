//! Network layer for QubeDB
//! 
//! Provides gRPC and REST APIs for:
//! - SQL queries
//! - GraphQL queries
//! - Vector search
//! - Graph operations

use crate::error::{QubeError, QubeResult};
use crate::query::QueryEngine;
use crate::storage::StorageEngine;
use tonic::{transport::Server, Request, Response, Status};
use std::net::SocketAddr;

/// gRPC service for QubeDB
pub struct QubeDBService {
    query_engine: QueryEngine,
    storage_engine: StorageEngine,
}

impl QubeDBService {
    pub fn new(storage_engine: StorageEngine) -> Self {
        QubeDBService {
            query_engine: QueryEngine::new(),
            storage_engine,
        }
    }
}

/// Start the QubeDB server
pub async fn start_server(addr: SocketAddr, storage_path: &str) -> QubeResult<()> {
    let storage_engine = StorageEngine::new(storage_path)?;
    let service = QubeDBService::new(storage_engine);
    
    // TODO: Implement actual gRPC service
    println!("QubeDB server starting on {}", addr);
    
    // Placeholder - actual server implementation will be added
    Ok(())
}

/// REST API handler
pub struct RestApiHandler {
    query_engine: QueryEngine,
    storage_engine: StorageEngine,
}

impl RestApiHandler {
    pub fn new(storage_engine: StorageEngine) -> Self {
        RestApiHandler {
            query_engine: QueryEngine::new(),
            storage_engine,
        }
    }
    
    /// Handle SQL query via REST
    pub async fn handle_sql_query(&self, sql: &str) -> QubeResult<serde_json::Value> {
        let result = self.query_engine.execute_sql(sql).await?;
        
        // Convert QueryResult to JSON
        let json_result = serde_json::json!({
            "columns": result.columns,
            "rows": result.rows,
            "affected_rows": result.affected_rows,
            "execution_time_ms": result.execution_time.as_millis()
        });
        
        Ok(json_result)
    }
    
    /// Handle GraphQL query via REST
    pub async fn handle_graphql_query(&self, query: &str) -> QubeResult<serde_json::Value> {
        let result = self.query_engine.execute_graphql(query).await?;
        
        let json_result = serde_json::json!({
            "data": result.rows,
            "execution_time_ms": result.execution_time.as_millis()
        });
        
        Ok(json_result)
    }
    
    /// Handle vector search via REST
    pub async fn handle_vector_search(&self, collection: &str, query_vector: Vec<f32>, limit: usize) -> QubeResult<serde_json::Value> {
        let result = self.query_engine.execute_vector_search(collection, &query_vector, limit).await?;
        
        let json_result = serde_json::json!({
            "results": result.rows,
            "execution_time_ms": result.execution_time.as_millis()
        });
        
        Ok(json_result)
    }
}
