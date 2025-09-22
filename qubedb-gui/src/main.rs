//! QubeDB Desktop GUI Application
//! Modern desktop interface for QubeDB database management

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use qubedb_core::embedded::EmbeddedQubeDB;
use qubedb_core::types::{Row, Value, QueryResult};

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConnection {
    name: String,
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    ssl: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryRequest {
    sql: String,
    connection_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryResponse {
    success: bool,
    data: Option<QueryResult>,
    error: Option<String>,
    execution_time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct TableInfo {
    name: String,
    columns: Vec<ColumnInfo>,
    row_count: u64,
    size_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ColumnInfo {
    name: String,
    data_type: String,
    nullable: bool,
    primary_key: bool,
}

// Global state untuk menyimpan koneksi database
struct AppState {
    connections: HashMap<String, EmbeddedQubeDB>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            connections: HashMap::new(),
        })
        .invoke_handler(tauri::generate_handler![
            connect_database,
            disconnect_database,
            execute_query,
            get_tables,
            get_table_info,
            get_connections,
            create_table,
            insert_data,
            update_data,
            delete_data
        ])
        .setup(|app| {
            // Initialize logging
            tracing_subscriber::fmt::init();
            
            // Show main window
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Connect to a QubeDB database
#[tauri::command]
async fn connect_database(
    connection: DatabaseConnection,
    state: tauri::State<'_, tauri::async_runtime::Mutex<AppState>>,
) -> Result<String, String> {
    println!("🔌 Connecting to database: {}", connection.name);
    
    let connection_id = format!("{}_{}", connection.name, chrono::Utc::now().timestamp());
    
    // Create embedded database connection
    let db_path = format!("./databases/{}", connection.database);
    let db = EmbeddedQubeDB::open(&db_path)
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    // Store connection
    let mut app_state = state.lock().await;
    app_state.connections.insert(connection_id.clone(), db);
    
    println!("✅ Connected to database: {}", connection.name);
    Ok(connection_id)
}

/// Disconnect from database
#[tauri::command]
async fn disconnect_database(
    connection_id: String,
    state: tauri::State<'_, tauri::async_runtime::Mutex<AppState>>,
) -> Result<(), String> {
    println!("🔌 Disconnecting from database: {}", connection_id);
    
    let mut app_state = state.lock().await;
    app_state.connections.remove(&connection_id);
    
    println!("✅ Disconnected from database: {}", connection_id);
    Ok(())
}

/// Execute SQL query
#[tauri::command]
async fn execute_query(
    request: QueryRequest,
    state: tauri::State<'_, tauri::async_runtime::Mutex<AppState>>,
) -> Result<QueryResponse, String> {
    let start_time = std::time::Instant::now();
    
    let app_state = state.lock().await;
    let db = app_state.connections.get(&request.connection_id)
        .ok_or("Database connection not found")?;
    
    println!("🔍 Executing query: {}", request.sql);
    
    match db.execute(&request.sql).await {
        Ok(result) => {
            let execution_time = start_time.elapsed().as_millis() as u64;
            println!("✅ Query executed successfully in {}ms", execution_time);
            
            Ok(QueryResponse {
                success: true,
                data: Some(result),
                error: None,
                execution_time,
            })
        }
        Err(e) => {
            let execution_time = start_time.elapsed().as_millis() as u64;
            println!("❌ Query failed: {}", e);
            
            Ok(QueryResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
                execution_time,
            })
        }
    }
}

/// Get list of tables
#[tauri::command]
async fn get_tables(
    connection_id: String,
    state: tauri::State<'_, tauri::async_runtime::Mutex<AppState>>,
) -> Result<Vec<String>, String> {
    let app_state = state.lock().await;
    let _db = app_state.connections.get(&connection_id)
        .ok_or("Database connection not found")?;
    
    // In a real implementation, this would query the database for table names
    // For now, return sample tables
    Ok(vec![
        "users".to_string(),
        "products".to_string(),
        "orders".to_string(),
        "categories".to_string(),
    ])
}

/// Get table information
#[tauri::command]
async fn get_table_info(
    connection_id: String,
    table_name: String,
    state: tauri::State<'_, tauri::async_runtime::Mutex<AppState>>,
) -> Result<TableInfo, String> {
    let app_state = state.lock().await;
    let _db = app_state.connections.get(&connection_id)
        .ok_or("Database connection not found")?;
    
    // In a real implementation, this would query the database for table schema
    // For now, return sample table info
    Ok(TableInfo {
        name: table_name.clone(),
        columns: vec![
            ColumnInfo {
                name: "id".to_string(),
                data_type: "INTEGER".to_string(),
                nullable: false,
                primary_key: true,
            },
            ColumnInfo {
                name: "name".to_string(),
                data_type: "VARCHAR(255)".to_string(),
                nullable: false,
                primary_key: false,
            },
            ColumnInfo {
                name: "email".to_string(),
                data_type: "VARCHAR(255)".to_string(),
                nullable: true,
                primary_key: false,
            },
        ],
        row_count: 1000,
        size_bytes: 1024 * 1024, // 1MB
    })
}

/// Get active connections
#[tauri::command]
async fn get_connections(
    state: tauri::State<'_, tauri::async_runtime::Mutex<AppState>>,
) -> Result<Vec<String>, String> {
    let app_state = state.lock().await;
    Ok(app_state.connections.keys().cloned().collect())
}

/// Create new table
#[tauri::command]
async fn create_table(
    connection_id: String,
    table_name: String,
    columns: Vec<ColumnInfo>,
    state: tauri::State<'_, tauri::async_runtime::Mutex<AppState>>,
) -> Result<(), String> {
    let app_state = state.lock().await;
    let db = app_state.connections.get(&connection_id)
        .ok_or("Database connection not found")?;
    
    // Build CREATE TABLE SQL
    let mut sql = format!("CREATE TABLE {} (", table_name);
    let column_defs: Vec<String> = columns.iter()
        .map(|col| {
            let mut def = format!("{} {}", col.name, col.data_type);
            if !col.nullable {
                def.push_str(" NOT NULL");
            }
            if col.primary_key {
                def.push_str(" PRIMARY KEY");
            }
            def
        })
        .collect();
    
    sql.push_str(&column_defs.join(", "));
    sql.push(')');
    
    println!("📊 Creating table: {}", sql);
    
    db.execute(&sql).await
        .map_err(|e| format!("Failed to create table: {}", e))?;
    
    println!("✅ Table created successfully: {}", table_name);
    Ok(())
}

/// Insert data into table
#[tauri::command]
async fn insert_data(
    connection_id: String,
    table_name: String,
    data: HashMap<String, serde_json::Value>,
    state: tauri::State<'_, tauri::async_runtime::Mutex<AppState>>,
) -> Result<(), String> {
    let app_state = state.lock().await;
    let db = app_state.connections.get(&connection_id)
        .ok_or("Database connection not found")?;
    
    // Convert JSON data to Row
    let mut row = HashMap::new();
    for (key, value) in data {
        let qubedb_value = match value {
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    Value::Int32(n.as_i64().unwrap() as i32)
                } else {
                    Value::Float64(n.as_f64().unwrap())
                }
            }
            serde_json::Value::Bool(b) => Value::Boolean(b),
            _ => Value::String(value.to_string()),
        };
        row.insert(key, qubedb_value);
    }
    
    println!("➕ Inserting data into table: {}", table_name);
    
    db.insert(&table_name, row)
        .map_err(|e| format!("Failed to insert data: {}", e))?;
    
    println!("✅ Data inserted successfully");
    Ok(())
}

/// Update data in table
#[tauri::command]
async fn update_data(
    connection_id: String,
    table_name: String,
    id: String,
    data: HashMap<String, serde_json::Value>,
    state: tauri::State<'_, tauri::async_runtime::Mutex<AppState>>,
) -> Result<(), String> {
    let app_state = state.lock().await;
    let db = app_state.connections.get(&connection_id)
        .ok_or("Database connection not found")?;
    
    // Convert JSON data to Row
    let mut row = HashMap::new();
    for (key, value) in data {
        let qubedb_value = match value {
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Number(n) => {
                if n.is_i64() {
                    Value::Int32(n.as_i64().unwrap() as i32)
                } else {
                    Value::Float64(n.as_f64().unwrap())
                }
            }
            serde_json::Value::Bool(b) => Value::Boolean(b),
            _ => Value::String(value.to_string()),
        };
        row.insert(key, qubedb_value);
    }
    
    println!("🔄 Updating data in table: {} with id: {}", table_name, id);
    
    db.update(&table_name, &id, row)
        .map_err(|e| format!("Failed to update data: {}", e))?;
    
    println!("✅ Data updated successfully");
    Ok(())
}

/// Delete data from table
#[tauri::command]
async fn delete_data(
    connection_id: String,
    table_name: String,
    id: String,
    state: tauri::State<'_, tauri::async_runtime::Mutex<AppState>>,
) -> Result<(), String> {
    let app_state = state.lock().await;
    let db = app_state.connections.get(&connection_id)
        .ok_or("Database connection not found")?;
    
    println!("🗑️ Deleting data from table: {} with id: {}", table_name, id);
    
    db.delete(&table_name, &id)
        .map_err(|e| format!("Failed to delete data: {}", e))?;
    
    println!("✅ Data deleted successfully");
    Ok(())
}
