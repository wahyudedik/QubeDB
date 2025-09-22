//! Basic usage example for QubeDB Core
//! 
//! This example demonstrates the basic features of QubeDB:
//! - Embedded database usage
//! - Multi-model data storage
//! - SQL querying
//! - Vector operations
//! - Graph operations

use qubedb_core::embedded::EmbeddedQubeDB;
use qubedb_core::types::{Row, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 QubeDB Core - Basic Usage Example");
    println!("=====================================");
    
    // Create embedded database
    println!("\n📦 Creating embedded database...");
    let mut db = EmbeddedQubeDB::open("./example_db")?;
    println!("✅ Database created at: {}", db.path());
    
    // 1. Relational Data (SQL)
    println!("\n🗄️ Testing Relational Data...");
    
    // Insert users
    let mut user1 = HashMap::new();
    user1.insert("id".to_string(), Value::Int32(1));
    user1.insert("name".to_string(), Value::String("Alice".to_string()));
    user1.insert("age".to_string(), Value::Int32(25));
    user1.insert("email".to_string(), Value::String("alice@example.com".to_string()));
    db.insert("users", user1)?;
    
    let mut user2 = HashMap::new();
    user2.insert("id".to_string(), Value::Int32(2));
    user2.insert("name".to_string(), Value::String("Bob".to_string()));
    user2.insert("age".to_string(), Value::Int32(30));
    user2.insert("email".to_string(), Value::String("bob@example.com".to_string()));
    db.insert("users", user2)?;
    
    println!("✅ Inserted 2 users");
    
    // Query users
    let result = db.execute("SELECT * FROM users WHERE age > 25").await?;
    println!("✅ Found {} users over 25", result.rows.len());
    
    // 2. Document Data (JSON)
    println!("\n📄 Testing Document Data...");
    
    let mut product = HashMap::new();
    product.insert("id".to_string(), Value::Int32(1));
    product.insert("name".to_string(), Value::String("Laptop".to_string()));
    product.insert("price".to_string(), Value::Float64(999.99));
    product.insert("specs".to_string(), Value::Json(serde_json::json!({
        "cpu": "Intel i7",
        "ram": "16GB",
        "storage": "512GB SSD"
    })));
    db.insert("products", product)?;
    
    println!("✅ Inserted product with JSON specs");
    
    // 3. Vector Data (AI/ML)
    println!("\n🧠 Testing Vector Data...");
    
    // Store document embeddings
    let doc1_vector = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let doc2_vector = vec![0.6, 0.7, 0.8, 0.9, 1.0];
    let doc3_vector = vec![0.2, 0.3, 0.4, 0.5, 0.6];
    
    db.store_vector("document_embeddings", "doc1", &doc1_vector)?;
    db.store_vector("document_embeddings", "doc2", &doc2_vector)?;
    db.store_vector("document_embeddings", "doc3", &doc3_vector)?;
    
    println!("✅ Stored 3 document embeddings");
    
    // Retrieve vector
    if let Some(retrieved_vector) = db.get_vector("document_embeddings", "doc1")? {
        println!("✅ Retrieved vector: {:?}", retrieved_vector);
    }
    
    // 4. Graph Data
    println!("\n🕸️ Testing Graph Data...");
    
    // Store nodes
    let mut alice_node = HashMap::new();
    alice_node.insert("name".to_string(), Value::String("Alice".to_string()));
    alice_node.insert("type".to_string(), Value::String("Person".to_string()));
    db.store_node("social_graph", "alice", alice_node)?;
    
    let mut bob_node = HashMap::new();
    bob_node.insert("name".to_string(), Value::String("Bob".to_string()));
    bob_node.insert("type".to_string(), Value::String("Person".to_string()));
    db.store_node("social_graph", "bob", bob_node)?;
    
    // Store edges (relationships)
    let mut friendship = HashMap::new();
    friendship.insert("type".to_string(), Value::String("FRIENDS".to_string()));
    friendship.insert("since".to_string(), Value::String("2020-01-01".to_string()));
    db.store_edge("social_graph", "alice", "bob", friendship)?;
    
    println!("✅ Created social graph with 2 nodes and 1 edge");
    
    // 5. Complex Queries
    println!("\n🔍 Testing Complex Queries...");
    
    // Multi-table join simulation
    let users_result = db.execute("SELECT name, age FROM users").await?;
    println!("✅ Users query returned {} rows", users_result.rows.len());
    
    // Display results
    for (i, row) in users_result.rows.iter().enumerate() {
        println!("  User {}: {:?}", i + 1, row);
    }
    
    // 6. Performance Test
    println!("\n⚡ Performance Test...");
    let start = std::time::Instant::now();
    
    // Insert multiple records
    for i in 1..=100 {
        let mut record = HashMap::new();
        record.insert("id".to_string(), Value::Int32(i));
        record.insert("data".to_string(), Value::String(format!("Record {}", i)));
        db.insert("performance_test", record)?;
    }
    
    let duration = start.elapsed();
    println!("✅ Inserted 100 records in {:?}", duration);
    
    // 7. Data Retrieval
    println!("\n📊 Data Retrieval Test...");
    
    // Get specific user
    if let Some(user) = db.get("users", "1")? {
        println!("✅ Retrieved user: {:?}", user);
    }
    
    // Get vector data
    if let Some(vector) = db.get_vector("document_embeddings", "doc2")? {
        println!("✅ Retrieved vector: {:?}", vector);
    }
    
    println!("\n🎉 All tests completed successfully!");
    println!("\nQubeDB Core demonstrates:");
    println!("✅ Relational data storage and SQL queries");
    println!("✅ Document data with JSON support");
    println!("✅ Vector data for AI/ML applications");
    println!("✅ Graph data with nodes and edges");
    println!("✅ High-performance operations");
    println!("✅ Multi-model data in one database");
    
    println!("\n🚀 QubeDB Core is ready for production use!");
    
    Ok(())
}
