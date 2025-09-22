# 📚 QubeDB Documentation

## 🦀 QubeDB - Next Generation Multi-Model Database

QubeDB adalah database multi-model modern yang ditulis dalam Rust yang menggabungkan kekuatan database relational, document, graph, dan vector dalam satu sistem yang terpadu.

## 📋 Table of Contents

1. [Overview](#overview)
2. [Features](#features)
3. [Architecture](#architecture)
4. [Installation](#installation)
5. [Quick Start](#quick-start)
6. [Core Concepts](#core-concepts)
7. [Multi-Model Support](#multi-model-support)
8. [Drivers & Integrations](#drivers--integrations)
9. [Performance](#performance)
10. [Examples](#examples)
11. [API Reference](#api-reference)
12. [Troubleshooting](#troubleshooting)

## 🎯 Overview

QubeDB adalah database yang dirancang untuk era modern dengan dukungan:

- **Multi-Model**: Relational, Document, Graph, Vector dalam satu database
- **High Performance**: Dibangun dengan Rust untuk keamanan memori dan performa tinggi
- **Developer Friendly**: Driver untuk berbagai framework populer
- **Embedded Ready**: Dapat berjalan sebagai embedded database (seperti SQLite)
- **AI Ready**: Dukungan vector search untuk aplikasi AI/ML
- **Desktop GUI**: Interface desktop modern untuk database management

## ✨ Features

### 🗄️ Multi-Model Support
- **Relational (SQL)** - Full SQL support dengan ACID transactions
- **Document (JSON)** - Native JSON storage dan querying
- **Graph** - Nodes, edges, dan graph traversal
- **Vector (AI/ML)** - High-performance vector similarity search
- **Time-Series** - Optimized untuk time-series data

### 🚀 Performance
- **Memory Safety** - Dibangun dengan Rust untuk keamanan dan performa maksimal
- **Concurrent** - Arsitektur multi-threaded
- **Embedded** - Dapat berjalan sebagai embedded database (seperti SQLite)
- **Desktop GUI** - Interface desktop modern dengan Tauri

### 🔌 Developer Experience
- **Multiple Drivers** - PHP (Laravel), Python (Django), Java (Spring), Node.js, Go, Rust
- **SQL Standard** - Kompatibilitas ANSI SQL
- **Desktop GUI** - Interface desktop untuk database management
- **Embedded Mode** - Dapat digunakan sebagai library dalam aplikasi

## 🏗️ Architecture

### Core Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Query Engine  │    │  Storage Engine │    │  Index Engine   │
│                 │    │                 │    │                 │
│ • SQL Parser    │    │ • Multi-Model   │    │ • B-Tree        │
│ • Optimizer     │    │ • ACID          │    │ • Hash          │
│ • Executor      │    │ • Transactions  │    │ • Vector        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │   Desktop GUI   │
                    │                 │
                    │ • Tauri App     │
                    │ • Web Interface │
                    │ • Database Mgmt │
                    └─────────────────┘
```

### Data Flow

1. **Query Parsing** - SQL queries di-parse dan divalidasi
2. **Query Optimization** - Optimasi query untuk performa maksimal
3. **Execution Planning** - Generasi rencana eksekusi multi-model
4. **Storage Access** - Retrieval data efisien dari storage engine
5. **Result Formatting** - Format hasil untuk konsumsi client atau GUI

## 🚀 Installation

### Prerequisites

- Rust 1.90+ (untuk development)
- Cargo (package manager Rust)

### Install QubeDB Core

```bash
# Clone repository
git clone https://github.com/qubedb/qubedb.git
cd qubedb/qubedb-core

# Build
cargo build

# Run tests
cargo test

# Run example
cargo run --example basic_usage
```

### Add to Cargo.toml

```toml
[dependencies]
qubedb-core = "0.1.0"
```

## 🚀 Quick Start

### Embedded Database (SQLite-like)

```rust
use qubedb_core::embedded::EmbeddedQubeDB;
use qubedb_core::types::{Row, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Buka atau buat database
    let mut db = EmbeddedQubeDB::open("./my_database")?; 
    
    // Insert data
    let mut row = HashMap::new();
    row.insert("name".to_string(), Value::String("John Doe".to_string()));
    row.insert("age".to_string(), Value::Int32(30));
    db.insert("users", row)?;
    
    // Query data
    let result = db.execute("SELECT * FROM users WHERE age > 25").await?;
    println!("Found {} users", result.rows.len());
    
    // Vector search
    let vector = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    db.store_vector("embeddings", "doc1", &vector)?;
    
    // Graph operations
    let mut node_props = HashMap::new();
    node_props.insert("label".to_string(), Value::String("Person".to_string()));
    db.store_node("social_graph", "user1", node_props)?;
    
    Ok(())
}
```

### Desktop GUI Mode

```bash
# Install dependencies
install-everything.bat

# Build GUI
build-gui.bat

# Run GUI
run-gui-now.bat
```

## 🧠 Core Concepts

### Data Types

QubeDB mendukung berbagai tipe data:

```rust
use qubedb_core::types::Value;

// Numeric types
let int_val = Value::Int32(42);
let float_val = Value::Float64(3.14);

// Text dan binary
let string_val = Value::String("Hello World".to_string());
let binary_val = Value::Binary(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]);

// JSON documents
let json_val = Value::Json(serde_json::json!({"name": "John", "age": 30}));

// Vectors untuk AI/ML
let vector_val = Value::Vector(vec![0.1, 0.2, 0.3, 0.4, 0.5]);

// Timestamps
let timestamp_val = Value::Timestamp(1640995200);
```

### Storage Engine

Storage engine menyediakan persistensi data yang efisien:

```rust
use qubedb_core::storage::StorageEngine;

let storage = StorageEngine::new("./data")?;

// Store relational data
storage.put_row("users", "1", &row)?;

// Store vector data
storage.put_vector("embeddings", "doc1", &vector)?;

// Store graph data
storage.put_graph_node("graph", "node1", &properties)?;
```

### Query Engine

Execute SQL queries dengan dukungan parser lengkap:

```rust
use qubedb_core::query::QueryEngine;

let query_engine = QueryEngine::new();

// Parse SQL
let statement = query_engine.parse_sql("SELECT * FROM users WHERE age > 18")?;

// Execute query
let result = query_engine.execute_sql("SELECT * FROM users").await?;
```

## 🗄️ Multi-Model Support

### 1. Relational Data (SQL)

```rust
// Create table
let result = db.execute("CREATE TABLE users (id INT, name VARCHAR(100), age INT)").await?;

// Insert data
let result = db.execute("INSERT INTO users (id, name, age) VALUES (1, 'Alice', 25)").await?;

// Query data
let result = db.execute("SELECT * FROM users WHERE age > 20").await?;
```

### 2. Document Data (JSON)

```rust
// Store JSON document
let mut product = HashMap::new();
product.insert("id".to_string(), Value::Int32(1));
product.insert("name".to_string(), Value::String("Laptop".to_string()));
product.insert("specs".to_string(), Value::Json(serde_json::json!({
    "cpu": "Intel i7",
    "ram": "16GB",
    "storage": "512GB SSD"
})));
db.insert("products", product)?;
```

### 3. Vector Data (AI/ML)

```rust
// Store document embeddings
let doc1_vector = vec![0.1, 0.2, 0.3, 0.4, 0.5];
let doc2_vector = vec![0.6, 0.7, 0.8, 0.9, 1.0];

db.store_vector("document_embeddings", "doc1", &doc1_vector)?;
db.store_vector("document_embeddings", "doc2", &doc2_vector)?;

// Retrieve vector
if let Some(retrieved_vector) = db.get_vector("document_embeddings", "doc1")? {
    println!("Vector: {:?}", retrieved_vector);
}
```

### 4. Graph Data

```rust
// Store nodes
let mut alice_node = HashMap::new();
alice_node.insert("name".to_string(), Value::String("Alice".to_string()));
alice_node.insert("type".to_string(), Value::String("Person".to_string()));
db.store_node("social_graph", "alice", alice_node)?;

// Store edges (relationships)
let mut friendship = HashMap::new();
friendship.insert("type".to_string(), Value::String("FRIENDS".to_string()));
friendship.insert("since".to_string(), Value::String("2020-01-01".to_string()));
db.store_edge("social_graph", "alice", "bob", friendship)?;
```

## 🔌 Drivers & Integrations

### PHP/Laravel

```php
// PDO driver untuk Laravel
$pdo = new PDO('qubedb:host=localhost;port=8080;dbname=qubedb', $username, $password);
$stmt = $pdo->prepare('SELECT * FROM users WHERE age > ?');
$stmt->execute([18]);
$users = $stmt->fetchAll();
```

### Python/Django

```python
# Django ORM backend
from qubedb_django import QubeDBBackend

# Di settings.py
DATABASES = {
    'default': {
        'ENGINE': 'qubedb_django.backend',
        'NAME': 'qubedb',
        'HOST': 'localhost',
        'PORT': 8080,
    }
}
```

### Java/Spring Boot

```java
// JDBC driver untuk Spring Boot
@Configuration
public class DatabaseConfig {
    @Bean
    public DataSource dataSource() {
        return DataSourceBuilder.create()
            .driverClassName("com.qubedb.jdbc.Driver")
            .url("jdbc:qubedb://localhost:8080/qubedb")
            .build();
    }
}
```

### Node.js

```javascript
// Native driver untuk Node.js
const { QubeDB } = require('qubedb-node');

const db = new QubeDB({
    host: 'localhost',
    port: 8080,
    database: 'qubedb'
});

const result = await db.query('SELECT * FROM users WHERE age > ?', [18]);
console.log(result.rows);
```

### Go

```go
// database/sql driver untuk Go
import (
    "database/sql"
    _ "github.com/qubedb/go-driver"
)

db, err := sql.Open("qubedb", "qubedb://localhost:8080/qubedb")
if err != nil {
    log.Fatal(err)
}

rows, err := db.Query("SELECT * FROM users WHERE age > ?", 18)
if err != nil {
    log.Fatal(err)
}
defer rows.Close()
```

## ⚡ Performance

### Benchmark Results

- **Insert Performance**: 100 records dalam 634.3µs
- **Query Performance**: Sub-millisecond untuk simple queries
- **Vector Search**: Optimized untuk AI/ML workloads
- **Memory Usage**: Efficient dengan Rust's memory management

### Optimization Tips

1. **Use Indexes**: Buat index untuk kolom yang sering di-query
2. **Batch Operations**: Gunakan batch insert untuk data besar
3. **Connection Pooling**: Reuse connections untuk performa
4. **Vector Dimensions**: Optimize vector dimensions untuk search

## 📝 Examples

### Complete Example

```rust
use qubedb_core::embedded::EmbeddedQubeDB;
use qubedb_core::types::{Row, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create database
    let mut db = EmbeddedQubeDB::open("./example_db")?;
    
    // 1. Relational Data
    let mut user = HashMap::new();
    user.insert("id".to_string(), Value::Int32(1));
    user.insert("name".to_string(), Value::String("Alice".to_string()));
    user.insert("age".to_string(), Value::Int32(25));
    db.insert("users", user)?;
    
    // 2. Document Data
    let mut product = HashMap::new();
    product.insert("id".to_string(), Value::Int32(1));
    product.insert("name".to_string(), Value::String("Laptop".to_string()));
    product.insert("specs".to_string(), Value::Json(serde_json::json!({
        "cpu": "Intel i7",
        "ram": "16GB"
    })));
    db.insert("products", product)?;
    
    // 3. Vector Data
    let vector = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    db.store_vector("embeddings", "doc1", &vector)?;
    
    // 4. Graph Data
    let mut node = HashMap::new();
    node.insert("label".to_string(), Value::String("Person".to_string()));
    db.store_node("graph", "node1", node)?;
    
    // Query data
    let result = db.execute("SELECT * FROM users").await?;
    println!("Found {} users", result.rows.len());
    
    Ok(())
}
```

## 📖 API Reference

### EmbeddedQubeDB

#### Methods

- `open(path)` - Buka atau buat database
- `execute(sql)` - Execute SQL query
- `insert(table, row)` - Insert row ke table
- `get(table, id)` - Get row by ID
- `update(table, id, row)` - Update row
- `delete(table, id)` - Delete row
- `store_vector(collection, id, vector)` - Store vector
- `get_vector(collection, id)` - Get vector
- `store_node(graph, node_id, properties)` - Store graph node
- `store_edge(graph, from, to, properties)` - Store graph edge

### StorageEngine

#### Methods

- `new(path)` - Create storage engine
- `put_row(table, key, row)` - Store row
- `get_row(table, key)` - Get row
- `delete_row(table, key)` - Delete row
- `put_vector(collection, id, vector)` - Store vector
- `get_vector(collection, id)` - Get vector

### QueryEngine

#### Methods

- `new()` - Create query engine
- `parse_sql(sql)` - Parse SQL
- `execute_sql(sql)` - Execute SQL
- `execute_vector_search(collection, vector, limit)` - Vector search

## 🔧 Troubleshooting

### Common Issues

#### 1. Build Errors

```bash
# Error: missing libclang
# Solution: Install Visual Studio Build Tools
# Or use simplified build without RocksDB
```

#### 2. Connection Issues

```rust
// Error: Not connected to database
// Solution: Call connect() method first
let mut connection = PDOConnection::new(config);
connection.connect()?;
```

#### 3. Performance Issues

```rust
// Error: Slow queries
// Solution: Add indexes
db.execute("CREATE INDEX idx_age ON users(age)").await?;
```

### Debug Mode

```rust
// Enable debug logging
use tracing_subscriber;

tracing_subscriber::fmt::init();
```

## 📞 Support

- 📧 Email: support@qubedb.com
- 💬 Discord: [Join our community](https://discord.gg/qubedb)
- 📖 Documentation: [docs.qubedb.com](https://docs.qubedb.com)
- 🐛 Issues: [GitHub Issues](https://github.com/qubedb/qubedb/issues)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**QubeDB** - The future of databases is here! 🚀
