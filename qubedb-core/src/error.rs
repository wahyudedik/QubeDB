//! Error types for QubeDB

use thiserror::Error;

/// Main error type for QubeDB operations
#[derive(Error, Debug)]
pub enum QubeError {
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Query parsing error: {0}")]
    QueryParse(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Index error: {0}")]
    Index(String),

    #[error("Vector search error: {0}")]
    VectorSearch(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Database not found: {0}")]
    DatabaseNotFound(String),

    #[error("Table not found: {0}")]
    TableNotFound(String),

    #[error("Column not found: {0}")]
    ColumnNotFound(String),

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("Transaction error: {0}")]
    Transaction(String),
}

/// Result type alias for QubeDB operations
pub type QubeResult<T> = Result<T, QubeError>;
