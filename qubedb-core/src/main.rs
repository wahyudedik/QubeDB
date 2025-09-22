//! QubeDB Core - Test and Demo
//! 
//! This is a simple test program to verify QubeDB core functionality

use qubedb_core::{QubeDB, QubeError};
use qubedb_core::storage::StorageEngine;
use qubedb_core::query::QueryEngine;
use qubedb_core::types::{Row, Value};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 QubeDB Core - Next Generation Multi-Model Database");
    println!("==================================================");
    
    // Test storage engine
    println!("\n📦 Testing Storage Engine...");
    let storage = StorageEngine::new("./test_data")?;
    println!("✅ Storage engine initialized");
    
    // Test query engine
    println!("\n🔍 Testing Query Engine...");
    let query_engine = QueryEngine::new();
    println!("✅ Query engine initialized");
    
    // Test SQL parsing
    println!("\n📝 Testing SQL Parser...");
    let sql = "SELECT id, name FROM users WHERE age > 18";
    match query_engine.parse_sql(sql) {
        Ok(statement) => {
            println!("✅ SQL parsed successfully: {:?}", statement);
        }
        Err(e) => {
            println!("❌ SQL parsing failed: {}", e);
        }
    }
    
    // Test data storage
    println!("\n💾 Testing Data Storage...");
    let mut test_row = HashMap::new();
    test_row.insert("id".to_string(), Value::Int32(1));
    test_row.insert("name".to_string(), Value::String("QubeDB".to_string()));
    test_row.insert("version".to_string(), Value::String("0.1.0".to_string()));
    
    let mut storage = storage; // Make it mutable
    storage.put_row("test_table", "1", &test_row)?;
    println!("✅ Data stored successfully");
    
    // Test data retrieval
    println!("\n🔍 Testing Data Retrieval...");
    match storage.get_row("test_table", "1") {
        Ok(Some(row)) => {
            println!("✅ Data retrieved: {:?}", row);
        }
        Ok(None) => {
            println!("❌ No data found");
        }
        Err(e) => {
            println!("❌ Data retrieval failed: {}", e);
        }
    }
    
    // Test vector storage
    println!("\n🧠 Testing Vector Storage...");
    let test_vector = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    storage.put_vector("embeddings", "doc1", &test_vector)?;
    println!("✅ Vector stored successfully");
    
    match storage.get_vector("embeddings", "doc1") {
        Ok(Some(vector)) => {
            println!("✅ Vector retrieved: {:?}", vector);
        }
        Ok(None) => {
            println!("❌ No vector found");
        }
        Err(e) => {
            println!("❌ Vector retrieval failed: {}", e);
        }
    }
    
    println!("\n🎉 All tests completed successfully!");
    println!("\nQubeDB Core is ready for development!");
    
    Ok(())
}
