//! Simple QubeDB usage example
//! 
//! This example shows how to use QubeDB for basic database operations:
//! - Create database
//! - Insert data
//! - Query data
//! - Vector operations
//! - Graph operations

use qubedb_core::embedded::EmbeddedQubeDB;
use qubedb_core::types::Value;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 QubeDB Simple Usage Example");
    println!("================================");
    
    // Initialize logger
    qubedb_core::logging::init_logger(qubedb_core::logging::LoggerConfig::default())?;
    
    // Create or open database
    println!("\n📁 Creating database...");
    let mut db = EmbeddedQubeDB::open("./example_db")?;
    println!("✅ Database created: ./example_db");
    
    // 1. Relational Data (SQL-like)
    println!("\n🗄️  Testing Relational Data...");
    
    // Insert users
    let mut user1 = HashMap::new();
    user1.insert("id".to_string(), Value::Int32(1));
    user1.insert("name".to_string(), Value::String("Alice".to_string()));
    user1.insert("age".to_string(), Value::Int32(25));
    user1.insert("email".to_string(), Value::String("alice@example.com".to_string()));
    
    let mut user2 = HashMap::new();
    user2.insert("id".to_string(), Value::Int32(2));
    user2.insert("name".to_string(), Value::String("Bob".to_string()));
    user2.insert("age".to_string(), Value::Int32(30));
    user2.insert("email".to_string(), Value::String("bob@example.com".to_string()));
    
    db.insert("users", user1)?;
    db.insert("users", user2)?;
    println!("✅ Inserted 2 users");
    
    // Query users
    let result = db.execute("SELECT * FROM users").await?;
    println!("✅ Query executed: Found {} rows", result.rows.len());
    
    // 2. Document Data (JSON)
    println!("\n📄 Testing Document Data...");
    
    let mut product = HashMap::new();
    product.insert("id".to_string(), Value::Int32(1));
    product.insert("name".to_string(), Value::String("Laptop".to_string()));
    product.insert("price".to_string(), Value::Float64(999.99));
    product.insert("specs".to_string(), Value::Json(serde_json::json!({
        "cpu": "Intel i7",
        "ram": "16GB",
        "storage": "512GB SSD",
        "os": "Windows 11"
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
    println!("\n🕸️  Testing Graph Data...");
    
    // Store nodes
    let mut alice_node = HashMap::new();
    alice_node.insert("name".to_string(), Value::String("Alice".to_string()));
    alice_node.insert("type".to_string(), Value::String("Person".to_string()));
    alice_node.insert("age".to_string(), Value::Int32(25));
    
    let mut bob_node = HashMap::new();
    bob_node.insert("name".to_string(), Value::String("Bob".to_string()));
    bob_node.insert("type".to_string(), Value::String("Person".to_string()));
    bob_node.insert("age".to_string(), Value::Int32(30));
    
    db.store_node("social_graph", "alice", alice_node)?;
    db.store_node("social_graph", "bob", bob_node)?;
    println!("✅ Stored 2 graph nodes");
    
    // Store edges (relationships)
    let mut friendship = HashMap::new();
    friendship.insert("type".to_string(), Value::String("FRIENDS".to_string()));
    friendship.insert("since".to_string(), Value::String("2020-01-01".to_string()));
    friendship.insert("strength".to_string(), Value::Float64(0.8));
    
    db.store_edge("social_graph", "alice", "bob", friendship)?;
    println!("✅ Stored friendship edge");
    
    // 5. Performance Test
    println!("\n⚡ Performance Test...");
    
    let start = std::time::Instant::now();
    
    // Insert multiple records
    for i in 1..=100 {
        let mut record = HashMap::new();
        record.insert("id".to_string(), Value::Int32(i));
        record.insert("name".to_string(), Value::String(format!("User{}", i)));
        record.insert("value".to_string(), Value::Float64(i as f64 * 3.14));
        
        db.insert("performance_test", record)?;
    }
    
    let duration = start.elapsed();
    println!("✅ Inserted 100 records in {:?}", duration);
    
    // Query performance
    let start = std::time::Instant::now();
    let result = db.execute("SELECT * FROM performance_test WHERE id > 50").await?;
    let duration = start.elapsed();
    println!("✅ Query executed in {:?}, found {} rows", duration, result.rows.len());
    
    // 6. Summary
    println!("\n📊 Summary");
    println!("==========");
    println!("✅ Relational data: Users table with SQL queries");
    println!("✅ Document data: Products with JSON specifications");
    println!("✅ Vector data: Document embeddings for AI/ML");
    println!("✅ Graph data: Social network with nodes and edges");
    println!("✅ Performance: 100 records inserted and queried");
    
    println!("\n🎉 QubeDB example completed successfully!");
    println!("Database location: ./example_db");
    
    Ok(())
}
