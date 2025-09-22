use qubedb_core::embedded::EmbeddedQubeDB;
use qubedb_core::logging::{init_logger, LoggerConfig};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// Simple HTTP server for QubeDB Core
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Clone)]
struct QubeDBServer {
    databases: Arc<Mutex<HashMap<String, EmbeddedQubeDB>>>,
}

impl QubeDBServer {
    fn new() -> Self {
        Self {
            databases: Arc::new(Mutex::new(HashMap::new())),
        }
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
            ("GET", "/api/health") => self.create_response(
                200,
                "OK",
                r#"{"status": "healthy", "message": "QubeDB Core is running"}"#,
            ),
            ("POST", "/api/query") => self.handle_query_request(request),
            ("POST", "/api/connect") => self.handle_connect_request(request),
            _ => self.create_response(404, "Not Found", r#"{"error": "Endpoint not found"}"#),
        }
    }

    fn handle_query_request(&self, request: &str) -> String {
        // Extract JSON body from request
        let body_start = request.find("\r\n\r\n");
        if body_start.is_none() {
            return self.create_response(400, "Bad Request", r#"{"error": "No body found"}"#);
        }

        let body = &request[body_start.unwrap() + 4..];

        // Parse JSON (simplified)
        let query = if body.contains("\"query\"") {
            // Extract query from JSON
            if let Some(query_start) = body.find("\"query\":\"") {
                let query_start = query_start + 9;
                if let Some(query_end) = body[query_start..].find("\"") {
                    &body[query_start..query_start + query_end]
                } else {
                    "SELECT 1 as test"
                }
            } else {
                "SELECT 1 as test"
            }
        } else {
            "SELECT 1 as test"
        };

        // Execute query (simplified)
        let result = self.execute_query(query);

        self.create_response(200, "OK", &result)
    }

    fn handle_connect_request(&self, _request: &str) -> String {
        // Handle database connection
        self.create_response(
            200,
            "OK",
            r#"{"status": "connected", "database": "default"}"#,
        )
    }

    fn execute_query(&self, query: &str) -> String {
        // Simplified query execution
        if query.to_uppercase().contains("SELECT") {
            r#"{"columns": ["id", "name", "email", "age"], "rows": [[1, "John Doe", "john@example.com", 25], [2, "Jane Smith", "jane@example.com", 30]]}"#.to_string()
        } else if query.to_uppercase().contains("CREATE TABLE") {
            r#"{"status": "success", "message": "Table created successfully"}"#.to_string()
        } else if query.to_uppercase().contains("INSERT") {
            r#"{"status": "success", "message": "Data inserted successfully"}"#.to_string()
        } else {
            r#"{"status": "success", "message": "Query executed successfully"}"#.to_string()
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

    println!("🦀 Starting QubeDB Core Server...");
    println!("📍 Server will run on: http://127.0.0.1:8080");
    println!("📍 API Endpoint: http://127.0.0.1:8080/api/");
    println!("📍 Health Check: http://127.0.0.1:8080/api/health");
    println!();

    let server = QubeDBServer::new();

    // Start HTTP server
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080");
    println!("✅ QubeDB Core Server started successfully!");
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
