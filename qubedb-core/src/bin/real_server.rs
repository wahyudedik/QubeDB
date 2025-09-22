use qubedb_core::storage::{KeyValueStore, StoreStats};
use qubedb_core::embedded_simple::EmbeddedQubeDB;
use qubedb_core::logging::{init_logger, LoggerConfig};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// HTTP server for QubeDB Core
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Clone)]
struct QubeDBServer {
    store: Arc<KeyValueStore>,
}

#[derive(Deserialize)]
struct PutRequest {
    key: String,
    value: String,
}

#[derive(Deserialize)]
struct GetRequest {
    key: String,
}

#[derive(Serialize)]
struct GetResponse {
    key: String,
    value: Option<String>,
    found: bool,
}

#[derive(Serialize)]
struct StatsResponse {
    store_stats: StoreStats,
    server_uptime: u64,
}

impl QubeDBServer {
    fn new(store: Arc<KeyValueStore>) -> Self {
        Self { store }
    }

    fn handle_request(&self, request: &str) -> String {
        // Parse HTTP request
        let lines: Vec<&str> = request.lines().collect();
        if lines.is_empty() {
            return self.create_response(400, "Bad Request", "Empty request");
        }

        let request_line = lines[0];
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() < 3 {
            return self.create_response(400, "Bad Request", "Invalid request line");
        }

        let method = parts[0];
        let path = parts[1];

        match (method, path) {
            ("GET", "/api/health") => {
                self.create_response(200, "OK", r#"{"status": "healthy", "message": "QubeDB Real Database is running"}"#)
            }
            ("GET", "/api/stats") => {
                self.handle_stats_request()
            }
            ("POST", "/api/put") => {
                self.handle_put_request(request)
            }
            ("POST", "/api/get") => {
                self.handle_get_request(request)
            }
            ("POST", "/api/delete") => {
                self.handle_delete_request(request)
            }
            ("POST", "/api/flush") => {
                self.handle_flush_request()
            }
            _ => {
                self.create_response(404, "Not Found", r#"{"error": "Endpoint not found"}"#)
            }
        }
    }

    fn handle_stats_request(&self) -> String {
        let rt = tokio::runtime::Runtime::new().unwrap();
        match rt.block_on(self.store.stats()) {
            Ok(stats) => {
                let response = StatsResponse {
                    store_stats: stats,
                    server_uptime: 0, // TODO: Track uptime
                };
                match serde_json::to_string(&response) {
                    Ok(json) => self.create_response(200, "OK", &json),
                    Err(e) => self.create_response(500, "Internal Server Error", &format!(r#"{{"error": "{}"}}"#, e)),
                }
            }
            Err(e) => self.create_response(500, "Internal Server Error", &format!(r#"{{"error": "{}"}}"#, e)),
        }
    }

    fn handle_put_request(&self, request: &str) -> String {
        // Extract JSON body from request
        let body_start = request.find("\r\n\r\n");
        if body_start.is_none() {
            return self.create_response(400, "Bad Request", r#"{"error": "No body found"}"#);
        }

        let body = &request[body_start.unwrap() + 4..];
        
        match serde_json::from_str::<PutRequest>(body) {
            Ok(put_req) => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                match rt.block_on(self.store.put(put_req.key.clone(), put_req.value)) {
                    Ok(_) => {
                        let response = format!(r#"{{"status": "success", "message": "Key '{}' stored successfully"}}"#, put_req.key);
                        self.create_response(200, "OK", &response)
                    }
                    Err(e) => {
                        let response = format!(r#"{{"error": "{}"}}"#, e);
                        self.create_response(500, "Internal Server Error", &response)
                    }
                }
            }
            Err(e) => {
                let response = format!(r#"{{"error": "Invalid JSON: {}"}}"#, e);
                self.create_response(400, "Bad Request", &response)
            }
        }
    }

    fn handle_get_request(&self, request: &str) -> String {
        // Extract JSON body from request
        let body_start = request.find("\r\n\r\n");
        if body_start.is_none() {
            return self.create_response(400, "Bad Request", r#"{"error": "No body found"}"#);
        }

        let body = &request[body_start.unwrap() + 4..];
        
        match serde_json::from_str::<GetRequest>(body) {
            Ok(get_req) => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                match rt.block_on(self.store.get(&get_req.key)) {
                    Ok(value) => {
                        let response = GetResponse {
                            key: get_req.key,
                            value: value.clone(),
                            found: value.is_some(),
                        };
                        match serde_json::to_string(&response) {
                            Ok(json) => self.create_response(200, "OK", &json),
                            Err(e) => self.create_response(500, "Internal Server Error", &format!(r#"{{"error": "{}"}}"#, e)),
                        }
                    }
                    Err(e) => {
                        let response = format!(r#"{{"error": "{}"}}"#, e);
                        self.create_response(500, "Internal Server Error", &response)
                    }
                }
            }
            Err(e) => {
                let response = format!(r#"{{"error": "Invalid JSON: {}"}}"#, e);
                self.create_response(400, "Bad Request", &response)
            }
        }
    }

    fn handle_delete_request(&self, request: &str) -> String {
        // Extract JSON body from request
        let body_start = request.find("\r\n\r\n");
        if body_start.is_none() {
            return self.create_response(400, "Bad Request", r#"{"error": "No body found"}"#);
        }

        let body = &request[body_start.unwrap() + 4..];
        
        match serde_json::from_str::<GetRequest>(body) {
            Ok(delete_req) => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                match rt.block_on(self.store.delete(&delete_req.key)) {
                    Ok(deleted) => {
                        let response = format!(r#"{{"status": "success", "message": "Key '{}' {}"}}"#, 
                            delete_req.key, 
                            if deleted { "deleted successfully" } else { "not found" }
                        );
                        self.create_response(200, "OK", &response)
                    }
                    Err(e) => {
                        let response = format!(r#"{{"error": "{}"}}"#, e);
                        self.create_response(500, "Internal Server Error", &response)
                    }
                }
            }
            Err(e) => {
                let response = format!(r#"{{"error": "Invalid JSON: {}"}}"#, e);
                self.create_response(400, "Bad Request", &response)
            }
        }
    }

    fn handle_flush_request(&self) -> String {
        let rt = tokio::runtime::Runtime::new().unwrap();
        match rt.block_on(self.store.force_flush()) {
            Ok(_) => {
                self.create_response(200, "OK", r#"{"status": "success", "message": "MemTable flushed to SSTable"}"#)
            }
            Err(e) => {
                let response = format!(r#"{{"error": "{}"}}"#, e);
                self.create_response(500, "Internal Server Error", &response)
            }
        }
    }

    fn create_response(&self, status_code: u16, status_text: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n{}",
            status_code,
            status_text,
            body.len(),
            body
        )
    }
}

fn main() {
    // Initialize logging
    let config = LoggerConfig::default();
    init_logger(config).expect("Failed to initialize logger");

    println!("🦀 Starting QubeDB Real Database Server...");
    println!("📍 Server will run on: http://localhost:8080");
    println!("📍 API Endpoint: http://localhost:8080/api/");
    println!("📍 Health Check: http://localhost:8080/api/health");
    println!("📍 Stats: http://localhost:8080/api/stats");
    println!();

    // Initialize KeyValueStore
    let store = match KeyValueStore::new("./data") {
        Ok(store) => {
            println!("✅ KeyValueStore initialized");
            Arc::new(store)
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize KeyValueStore: {}", e);
            return;
        }
    };

    // Recover from WAL
    let rt = tokio::runtime::Runtime::new().unwrap();
    if let Err(e) = rt.block_on(store.recover()) {
        eprintln!("❌ Failed to recover from WAL: {}", e);
        return;
    }

    let server = QubeDBServer::new(store);

    // Start HTTP server
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080");
    println!("✅ QubeDB Real Database Server started successfully!");
    println!("🔍 Listening for connections on port 8080...");
    println!("Press Ctrl+C to stop the server");
    println!();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let server = server.clone();
                thread::spawn(move || {
                    handle_client(stream, server);
                });
            }
            Err(e) => {
                eprintln!("❌ Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, server: QubeDBServer) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buffer[..size]);
            let response = server.handle_request(&request);

            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("❌ Error writing response: {}", e);
            }
        }
        Err(e) => {
            eprintln!("❌ Error reading from stream: {}", e);
        }
    }
}
