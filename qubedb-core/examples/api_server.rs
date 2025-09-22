//! Example: QubeDB API Server
//! Demonstrates REST and GraphQL API capabilities

use qubedb_core::embedded::EmbeddedQubeDB;
use qubedb_core::api::{ApiConfig, RestApiServer, GraphQLApiServer};
use qubedb_core::security::{SecurityConfig, SecurityManager};
use std::collections::HashMap;
use qubedb_core::types::{Row, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 QubeDB API Server Example");
    println!("=============================");

    // Create database
    let db = EmbeddedQubeDB::open("./api_example_db")?;
    
    // Setup some sample data
    setup_sample_data(&db).await?;

    // Configure API
    let api_config = ApiConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        enable_cors: true,
        enable_auth: true,
        max_request_size: 10 * 1024 * 1024,
    };

    // Configure security
    let security_config = SecurityConfig {
        enable_tls: false,
        enable_auth: true,
        enable_rbac: true,
        jwt_secret: Some("your-secret-key".to_string()),
        ..Default::default()
    };

    let mut security_manager = SecurityManager::new(security_config);
    
    // Create sample users
    let admin_user = security_manager.create_user(
        "admin".to_string(),
        Some("admin@qubedb.com".to_string()),
        vec!["admin".to_string()],
    )?;

    let regular_user = security_manager.create_user(
        "user".to_string(),
        Some("user@qubedb.com".to_string()),
        vec!["user".to_string()],
    )?;

    println!("👥 Created users:");
    println!("  - Admin: {} (roles: {:?})", admin_user.username, admin_user.roles);
    println!("  - User: {} (roles: {:?})", regular_user.username, regular_user.roles);

    // Create API servers
    let rest_server = RestApiServer::new(api_config.clone(), db.clone());
    let graphql_server = GraphQLApiServer::new(api_config.clone(), db.clone());

    println!("\n📡 Starting API servers...");
    
    // Start REST API
    println!("🌐 REST API Server:");
    rest_server.start().await?;
    
    // Start GraphQL API
    println!("\n🔮 GraphQL API Server:");
    graphql_server.start().await?;

    println!("\n✅ API servers started successfully!");
    println!("\n📋 Available endpoints:");
    println!("  REST API:");
    println!("    GET  http://127.0.0.1:8080/health");
    println!("    POST http://127.0.0.1:8080/query");
    println!("    GET  http://127.0.0.1:8080/tables");
    println!("    POST http://127.0.0.1:8080/vectors/search");
    println!("  GraphQL API:");
    println!("    POST http://127.0.0.1:8080/graphql");
    println!("    GET  http://127.0.0.1:8080/playground");

    println!("\n🔐 Security features:");
    println!("  - JWT Authentication");
    println!("  - Role-based Access Control (RBAC)");
    println!("  - Permission-based authorization");
    println!("  - User management");

    println!("\n🎯 Test the APIs:");
    println!("  curl -X GET http://127.0.0.1:8080/health");
    println!("  curl -X POST http://127.0.0.1:8080/query -d '{\"sql\":\"SELECT * FROM users\"}'");

    Ok(())
}

async fn setup_sample_data(db: &EmbeddedQubeDB) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Setting up sample data...");

    // Create users table
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

    // Create products table
    let mut product1 = HashMap::new();
    product1.insert("id".to_string(), Value::Int32(1));
    product1.insert("name".to_string(), Value::String("Laptop".to_string()));
    product1.insert("price".to_string(), Value::Float64(999.99));
    product1.insert("category".to_string(), Value::String("Electronics".to_string()));
    db.insert("products", product1)?;

    // Store some vectors
    let vector1 = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let vector2 = vec![0.6, 0.7, 0.8, 0.9, 1.0];
    db.store_vector("embeddings", "doc1", &vector1)?;
    db.store_vector("embeddings", "doc2", &vector2)?;

    // Create graph data
    let mut node1 = HashMap::new();
    node1.insert("label".to_string(), Value::String("Person".to_string()));
    node1.insert("name".to_string(), Value::String("Alice".to_string()));
    db.store_node("social_graph", "alice", node1)?;

    let mut node2 = HashMap::new();
    node2.insert("label".to_string(), Value::String("Person".to_string()));
    node2.insert("name".to_string(), Value::String("Bob".to_string()));
    db.store_node("social_graph", "bob", node2)?;

    let mut edge = HashMap::new();
    edge.insert("type".to_string(), Value::String("FRIENDS".to_string()));
    edge.insert("since".to_string(), Value::String("2020-01-01".to_string()));
    db.store_edge("social_graph", "alice", "bob", edge)?;

    println!("✅ Sample data created:");
    println!("  - 2 users in 'users' table");
    println!("  - 1 product in 'products' table");
    println!("  - 2 vectors in 'embeddings' collection");
    println!("  - 2 nodes and 1 edge in 'social_graph'");

    Ok(())
}
