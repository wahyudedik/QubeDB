//! Core data types for QubeDB

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported data types in QubeDB
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    /// Integer types
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,

    /// Floating point types
    Float32,
    Float64,

    /// Text types
    String,
    Text,

    /// Binary types
    Binary,
    Blob,

    /// JSON document
    Json,

    /// Vector for AI/ML
    Vector {
        dimensions: usize,
    },

    /// Graph node/edge
    GraphNode,
    GraphEdge,

    /// Time series
    Timestamp,
    Date,
    Time,

    /// Boolean
    Boolean,
}

/// Column definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default_value: Option<Value>,
    pub primary_key: bool,
    pub unique: bool,
    pub index: bool,
}

/// Table definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
    pub constraints: Vec<Constraint>,
}

/// Index definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub index_type: IndexType,
    pub unique: bool,
}

/// Index types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexType {
    BTree,
    Hash,
    Vector, // For AI/ML vector search
    FullText,
    Spatial,
}

/// Constraint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub name: String,
    pub constraint_type: ConstraintType,
    pub columns: Vec<String>,
}

/// Constraint types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey {
        referenced_table: String,
        referenced_column: String,
    },
    Unique,
    Check {
        expression: String,
    },
}

/// Value types that can be stored
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Null,
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    String(String),
    Binary(Vec<u8>),
    Json(serde_json::Value),
    Vector(Vec<f32>),
    Boolean(bool),
    Timestamp(i64),
}

/// Row in a table
pub type Row = HashMap<String, Value>;

/// Query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
    pub affected_rows: usize,
    #[serde(skip)]
    pub execution_time: std::time::Duration,
}

// Manual implementations for Value to handle float types
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Int8(a), Value::Int8(b)) => a == b,
            (Value::Int16(a), Value::Int16(b)) => a == b,
            (Value::Int32(a), Value::Int32(b)) => a == b,
            (Value::Int64(a), Value::Int64(b)) => a == b,
            (Value::UInt8(a), Value::UInt8(b)) => a == b,
            (Value::UInt16(a), Value::UInt16(b)) => a == b,
            (Value::UInt32(a), Value::UInt32(b)) => a == b,
            (Value::UInt64(a), Value::UInt64(b)) => a == b,
            (Value::Float32(a), Value::Float32(b)) => (a - b).abs() < f32::EPSILON,
            (Value::Float64(a), Value::Float64(b)) => (a - b).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Binary(a), Value::Binary(b)) => a == b,
            (Value::Json(a), Value::Json(b)) => a == b,
            (Value::Vector(a), Value::Vector(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter()
                    .zip(b.iter())
                    .all(|(x, y)| (x - y).abs() < f32::EPSILON)
            }
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Timestamp(a), Value::Timestamp(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Null => 0.hash(state),
            Value::Int8(v) => v.hash(state),
            Value::Int16(v) => v.hash(state),
            Value::Int32(v) => v.hash(state),
            Value::Int64(v) => v.hash(state),
            Value::UInt8(v) => v.hash(state),
            Value::UInt16(v) => v.hash(state),
            Value::UInt32(v) => v.hash(state),
            Value::UInt64(v) => v.hash(state),
            Value::Float32(v) => v.to_bits().hash(state),
            Value::Float64(v) => v.to_bits().hash(state),
            Value::String(v) => v.hash(state),
            Value::Binary(v) => v.hash(state),
            Value::Json(v) => v.to_string().hash(state),
            Value::Vector(v) => {
                for f in v {
                    f.to_bits().hash(state);
                }
            }
            Value::Boolean(v) => v.hash(state),
            Value::Timestamp(v) => v.hash(state),
        }
    }
}
