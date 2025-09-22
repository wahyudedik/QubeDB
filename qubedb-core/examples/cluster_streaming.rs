//! Example: QubeDB Cluster & Streaming
//! Demonstrates distributed database capabilities with streaming integration

use qubedb_core::cluster::{ClusterConfig, ClusterManager, NodeRole, NodeStatus, Peer};
use qubedb_core::streaming::{StreamingConfig, StreamingManager, StreamingPlatform, StreamingMessage};
use qubedb_core::embedded::EmbeddedQubeDB;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 QubeDB Cluster & Streaming Example");
    println!("====================================");

    // 1. Setup Cluster
    println!("\n📊 Setting up QubeDB Cluster...");
    setup_cluster().await?;

    // 2. Setup Streaming
    println!("\n📡 Setting up Streaming Integration...");
    setup_streaming().await?;

    // 3. Demonstrate Integration
    println!("\n🔄 Demonstrating Cluster + Streaming Integration...");
    demonstrate_integration().await?;

    println!("\n✅ Cluster & Streaming example completed successfully!");
    Ok(())
}

async fn setup_cluster() -> Result<(), Box<dyn std::error::Error>> {
    // Create cluster configuration
    let cluster_config = ClusterConfig {
        node_id: "node-1".to_string(),
        bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        peers: vec![
            Peer {
                id: "node-2".to_string(),
                address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
                role: NodeRole::Follower,
                status: NodeStatus::Healthy,
                last_seen: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            Peer {
                id: "node-3".to_string(),
                address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8082),
                role: NodeRole::Follower,
                status: NodeStatus::Healthy,
                last_seen: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
        ],
        replication_factor: 3,
        shard_count: 4,
        enable_auto_discovery: true,
        heartbeat_interval: 1000,
        election_timeout: 5000,
    };

    // Create cluster manager
    let mut cluster_manager = ClusterManager::new(cluster_config);
    
    // Start cluster
    cluster_manager.start().await?;

    // Add additional peers
    cluster_manager.add_peer(Peer {
        id: "node-4".to_string(),
        address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8083),
        role: NodeRole::Observer,
        status: NodeStatus::Joining,
        last_seen: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    });

    // Display cluster status
    let status = cluster_manager.get_cluster_status();
    println!("📊 Cluster Status:");
    println!("   Node ID: {}", status.node_id);
    println!("   Role: {:?}", status.role);
    println!("   Leader: {:?}", status.leader_id);
    println!("   Peers: {}", status.peer_count);
    println!("   Shards: {}", status.shard_count);
    println!("   Healthy Peers: {}", status.healthy_peers);

    Ok(())
}

async fn setup_streaming() -> Result<(), Box<dyn std::error::Error>> {
    // Create streaming configuration for Kafka
    let kafka_config = StreamingConfig {
        platform: StreamingPlatform::Kafka,
        brokers: vec![
            "localhost:9092".to_string(),
            "localhost:9093".to_string(),
        ],
        topics: vec![
            "qubedb-events".to_string(),
            "qubedb-changes".to_string(),
            "qubedb-analytics".to_string(),
        ],
        consumer_group: "qubedb-cluster".to_string(),
        enable_auto_commit: true,
        batch_size: 100,
        flush_interval_ms: 1000,
    };

    // Create streaming manager
    let mut streaming_manager = StreamingManager::new(kafka_config);
    
    // Start streaming
    streaming_manager.start().await?;

    // Create producers and consumers
    streaming_manager.create_producer("qubedb-events".to_string()).await?;
    streaming_manager.create_producer("qubedb-changes".to_string()).await?;
    streaming_manager.create_consumer(vec![
        "qubedb-events".to_string(),
        "qubedb-changes".to_string(),
    ]).await?;

    // Display streaming statistics
    let stats = streaming_manager.get_statistics();
    println!("📡 Streaming Statistics:");
    println!("   Platform: {:?}", stats.platform);
    println!("   Producers: {}", stats.producer_count);
    println!("   Consumers: {}", stats.consumer_count);
    println!("   Topics: {:?}", stats.topics);
    println!("   Brokers: {:?}", stats.brokers);

    Ok(())
}

async fn demonstrate_integration() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Demonstrating Cluster + Streaming Integration...");

    // Create database
    let db = EmbeddedQubeDB::open("./cluster_example_db")?;

    // Simulate data changes that trigger streaming events
    println!("📝 Simulating data changes...");
    
    // Insert user data
    let mut user = HashMap::new();
    user.insert("id".to_string(), qubedb_core::types::Value::Int32(1));
    user.insert("name".to_string(), qubedb_core::types::Value::String("Alice".to_string()));
    user.insert("email".to_string(), qubedb_core::types::Value::String("alice@example.com".to_string()));
    db.insert("users", user)?;

    // Simulate streaming message for this change
    let change_message = StreamingMessage {
        topic: "qubedb-changes".to_string(),
        partition: Some(0),
        offset: Some(1),
        key: Some("user:1".to_string()),
        value: serde_json::to_vec(&serde_json::json!({
            "operation": "INSERT",
            "table": "users",
            "key": "1",
            "data": {
                "id": 1,
                "name": "Alice",
                "email": "alice@example.com"
            }
        }))?,
        headers: HashMap::new(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    println!("📤 Streaming change event:");
    println!("   Topic: {}", change_message.topic);
    println!("   Key: {:?}", change_message.key);
    println!("   Value: {}", String::from_utf8_lossy(&change_message.value));

    // Simulate analytics event
    let analytics_message = StreamingMessage {
        topic: "qubedb-analytics".to_string(),
        partition: Some(1),
        offset: Some(1),
        key: Some("analytics:user_insert".to_string()),
        value: serde_json::to_vec(&serde_json::json!({
            "event_type": "user_insert",
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "metadata": {
                "table": "users",
                "operation": "INSERT",
                "cluster_node": "node-1"
            }
        }))?,
        headers: HashMap::new(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    println!("📊 Streaming analytics event:");
    println!("   Topic: {}", analytics_message.topic);
    println!("   Key: {:?}", analytics_message.key);
    println!("   Value: {}", String::from_utf8_lossy(&analytics_message.value));

    // Demonstrate cluster sharding
    println!("\n📦 Demonstrating cluster sharding...");
    
    // Simulate shard assignment for different keys
    let keys = vec!["user:1", "user:2", "user:3", "user:4"];
    for key in keys {
        // In a real implementation, this would use the actual sharding logic
        let hash = key.len() as u32; // Simplified hash
        let shard_id = hash % 4; // 4 shards
        println!("   Key '{}' -> Shard {}", key, shard_id);
    }

    // Demonstrate replication
    println!("\n🔄 Demonstrating replication...");
    println!("   Replicating changes to 3 nodes");
    println!("   Node 1: Primary (Leader)");
    println!("   Node 2: Replica (Follower)");
    println!("   Node 3: Replica (Follower)");
    println!("   Node 4: Observer (Read-only)");

    println!("\n✅ Integration demonstration completed!");
    println!("   - Data changes trigger streaming events");
    println!("   - Events are distributed across cluster nodes");
    println!("   - Analytics events provide real-time insights");
    println!("   - Sharding ensures data distribution");
    println!("   - Replication provides high availability");

    Ok(())
}
