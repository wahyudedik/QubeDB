use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Simple WAL Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WALEntry {
    timestamp: u64,
    operation: String,
    key: String,
    value: Option<String>,
}

/// Simple Key-Value Store with WAL
struct SimpleKVStore {
    data: Arc<Mutex<HashMap<String, String>>>,
    wal_file: String,
}

impl SimpleKVStore {
    fn new(data_dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        std::fs::create_dir_all(data_dir)?;
        
        let store = SimpleKVStore {
            data: Arc::new(Mutex::new(HashMap::new())),
            wal_file: format!("{}/wal.log", data_dir),
        };
        
        // Recover from WAL
        store.recover()?;
        
        Ok(store)
    }
    
    fn put(&self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        // Write to WAL first
        let entry = WALEntry {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            operation: "PUT".to_string(),
            key: key.clone(),
            value: Some(value.clone()),
        };
        
        self.write_to_wal(&entry)?;
        
        // Update in-memory data
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
        
        Ok(())
    }
    
    fn get(&self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let data = self.data.lock().unwrap();
        Ok(data.get(key).cloned())
    }
    
    fn delete(&self, key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        // Write to WAL first
        let entry = WALEntry {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            operation: "DELETE".to_string(),
            key: key.to_string(),
            value: None,
        };
        
        self.write_to_wal(&entry)?;
        
        // Remove from in-memory data
        let mut data = self.data.lock().unwrap();
        Ok(data.remove(key).is_some())
    }
    
    fn write_to_wal(&self, entry: &WALEntry) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.wal_file)?;
        
        let json_entry = serde_json::to_string(entry)?;
        writeln!(file, "{}", json_entry)?;
        file.sync_all()?;
        
        Ok(())
    }
    
    fn recover(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !std::path::Path::new(&self.wal_file).exists() {
            return Ok(());
        }
        
        let file = File::open(&self.wal_file)?;
        let reader = BufReader::new(file);
        
        let mut data = self.data.lock().unwrap();
        
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<WALEntry>(&line) {
                Ok(entry) => {
                    match entry.operation.as_str() {
                        "PUT" => {
                            if let Some(value) = entry.value {
                                data.insert(entry.key, value);
                            }
                        }
                        "DELETE" => {
                            data.remove(&entry.key);
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse WAL entry: {} - {}", line, e);
                }
            }
        }
        
        Ok(())
    }
    
    fn stats(&self) -> Result<StoreStats, Box<dyn std::error::Error>> {
        let data = self.data.lock().unwrap();
        let wal_size = if std::path::Path::new(&self.wal_file).exists() {
            std::fs::metadata(&self.wal_file)?.len()
        } else {
            0
        };
        
        Ok(StoreStats {
            total_keys: data.len(),
            wal_size,
            uptime: 0, // TODO: Track uptime
        })
    }
}

#[derive(Debug, Serialize)]
struct StoreStats {
    total_keys: usize,
    wal_size: u64,
    uptime: u64,
}

/// Simple HTTP Server
struct SimpleServer {
    store: Arc<SimpleKVStore>,
}

impl SimpleServer {
    fn new(store: Arc<SimpleKVStore>) -> Self {
        Self { store }
    }
    
    fn handle_request(&self, request: &str) -> String {
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
                match self.store.stats() {
                    Ok(stats) => {
                        match serde_json::to_string(&stats) {
                            Ok(json) => self.create_response(200, "OK", &json),
                            Err(e) => self.create_response(500, "Internal Server Error", &format!(r#"{{"error": "{}"}}"#, e)),
                        }
                    }
                    Err(e) => self.create_response(500, "Internal Server Error", &format!(r#"{{"error": "{}"}}"#, e)),
                }
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
            _ => {
                self.create_response(404, "Not Found", r#"{"error": "Endpoint not found"}"#)
            }
        }
    }
    
    fn handle_put_request(&self, request: &str) -> String {
        let body_start = request.find("\r\n\r\n");
        if body_start.is_none() {
            return self.create_response(400, "Bad Request", r#"{"error": "No body found"}"#);
        }
        
        let body = &request[body_start.unwrap() + 4..];
        
        #[derive(Deserialize)]
        struct PutRequest {
            key: String,
            value: String,
        }
        
        match serde_json::from_str::<PutRequest>(body) {
            Ok(put_req) => {
                match self.store.put(put_req.key.clone(), put_req.value) {
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
        let body_start = request.find("\r\n\r\n");
        if body_start.is_none() {
            return self.create_response(400, "Bad Request", r#"{"error": "No body found"}"#);
        }
        
        let body = &request[body_start.unwrap() + 4..];
        
        #[derive(Deserialize)]
        struct GetRequest {
            key: String,
        }
        
        match serde_json::from_str::<GetRequest>(body) {
            Ok(get_req) => {
                match self.store.get(&get_req.key) {
                    Ok(value) => {
                        let response = format!(r#"{{"key": "{}", "value": {:?}, "found": {}}}"#, 
                            get_req.key, 
                            value, 
                            value.is_some()
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
    
    fn handle_delete_request(&self, request: &str) -> String {
        let body_start = request.find("\r\n\r\n");
        if body_start.is_none() {
            return self.create_response(400, "Bad Request", r#"{"error": "No body found"}"#);
        }
        
        let body = &request[body_start.unwrap() + 4..];
        
        #[derive(Deserialize)]
        struct DeleteRequest {
            key: String,
        }
        
        match serde_json::from_str::<DeleteRequest>(body) {
            Ok(delete_req) => {
                match self.store.delete(&delete_req.key) {
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
    println!("🦀 Starting QubeDB Real Database Server...");
    println!("📍 Server will run on: http://localhost:8080");
    println!("📍 API Endpoint: http://localhost:8080/api/");
    println!("📍 Health Check: http://localhost:8080/api/health");
    println!("📍 Stats: http://localhost:8080/api/stats");
    println!();
    
    // Initialize Key-Value Store
    let store = match SimpleKVStore::new("./data") {
        Ok(store) => {
            println!("✅ Key-Value Store initialized");
            Arc::new(store)
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize Key-Value Store: {}", e);
            return;
        }
    };
    
    let server = SimpleServer::new(store);
    
    // Start HTTP server
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080");
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

fn handle_client(mut stream: std::net::TcpStream, server: SimpleServer) {
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
