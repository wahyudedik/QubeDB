//! QubeDB Logging System
//!
//! Comprehensive logging system for tracking all QubeDB operations,
//! errors, and performance metrics.

use crate::error::QubeError;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Log levels for different types of messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
}

/// Log categories for different types of operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogCategory {
    Installation,
    Connection,
    Query,
    Database,
    Table,
    Index,
    Vector,
    Graph,
    Performance,
    Security,
    Network,
    Storage,
    System,
    User,
    Application,
}

impl LogCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogCategory::Installation => "INSTALL",
            LogCategory::Connection => "CONNECT",
            LogCategory::Query => "QUERY",
            LogCategory::Database => "DATABASE",
            LogCategory::Table => "TABLE",
            LogCategory::Index => "INDEX",
            LogCategory::Vector => "VECTOR",
            LogCategory::Graph => "GRAPH",
            LogCategory::Performance => "PERF",
            LogCategory::Security => "SECURITY",
            LogCategory::Network => "NETWORK",
            LogCategory::Storage => "STORAGE",
            LogCategory::System => "SYSTEM",
            LogCategory::User => "USER",
            LogCategory::Application => "APP",
        }
    }
}

/// Log entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub category: LogCategory,
    pub message: String,
    pub details: Option<String>,
    pub error_code: Option<String>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub operation_id: Option<String>,
    pub duration_ms: Option<u64>,
    pub memory_usage: Option<u64>,
    pub cpu_usage: Option<f64>,
}

impl LogEntry {
    pub fn new(level: LogLevel, category: LogCategory, message: String) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            level,
            category,
            message,
            details: None,
            error_code: None,
            user_id: None,
            session_id: None,
            operation_id: None,
            duration_ms: None,
            memory_usage: None,
            cpu_usage: None,
        }
    }

    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_error_code(mut self, error_code: String) -> Self {
        self.error_code = Some(error_code);
        self
    }

    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_operation(mut self, operation_id: String) -> Self {
        self.operation_id = Some(operation_id);
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    pub fn with_metrics(mut self, memory_usage: u64, cpu_usage: f64) -> Self {
        self.memory_usage = Some(memory_usage);
        self.cpu_usage = Some(cpu_usage);
        self
    }
}

/// Logger configuration
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub log_file: String,
    pub max_file_size: u64,
    pub max_files: u32,
    pub log_level: LogLevel,
    pub enable_console: bool,
    pub enable_file: bool,
    pub enable_json: bool,
    pub enable_metrics: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_file: "qubedb.log".to_string(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 5,
            log_level: LogLevel::Info,
            enable_console: true,
            enable_file: true,
            enable_json: false,
            enable_metrics: true,
        }
    }
}

/// Main logger struct
pub struct Logger {
    config: LoggerConfig,
    file_handle: Mutex<Option<std::fs::File>>,
    metrics: Mutex<LogMetrics>,
}

/// Log metrics for performance tracking
#[derive(Debug, Default, Clone)]
pub struct LogMetrics {
    pub total_logs: u64,
    pub error_count: u64,
    pub warning_count: u64,
    pub info_count: u64,
    pub debug_count: u64,
    pub trace_count: u64,
    pub last_error: Option<String>,
    pub last_warning: Option<String>,
}

impl Logger {
    /// Create a new logger instance
    pub fn new(config: LoggerConfig) -> Result<Self, QubeError> {
        let logger = Self {
            config,
            file_handle: Mutex::new(None),
            metrics: Mutex::new(LogMetrics::default()),
        };

        // Initialize file handle if file logging is enabled
        if logger.config.enable_file {
            logger.initialize_file()?;
        }

        Ok(logger)
    }

    /// Initialize log file
    fn initialize_file(&self) -> Result<(), QubeError> {
        let mut file_handle = self.file_handle.lock().unwrap();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.config.log_file)
            .map_err(|e| QubeError::Io(e))?;

        *file_handle = Some(file);
        Ok(())
    }

    /// Log an entry
    pub fn log(&self, entry: LogEntry) -> Result<(), QubeError> {
        // Check if we should log this level
        if !self.should_log(&entry.level) {
            return Ok(());
        }

        // Update metrics
        self.update_metrics(&entry);

        // Log to console if enabled
        if self.config.enable_console {
            self.log_to_console(&entry);
        }

        // Log to file if enabled
        if self.config.enable_file {
            self.log_to_file(&entry)?;
        }

        Ok(())
    }

    /// Check if we should log this level
    fn should_log(&self, level: &LogLevel) -> bool {
        match (&self.config.log_level, level) {
            (LogLevel::Trace, _) => true,
            (
                LogLevel::Debug,
                LogLevel::Debug
                | LogLevel::Info
                | LogLevel::Warn
                | LogLevel::Error
                | LogLevel::Fatal,
            ) => true,
            (
                LogLevel::Info,
                LogLevel::Info | LogLevel::Warn | LogLevel::Error | LogLevel::Fatal,
            ) => true,
            (LogLevel::Warn, LogLevel::Warn | LogLevel::Error | LogLevel::Fatal) => true,
            (LogLevel::Error, LogLevel::Error | LogLevel::Fatal) => true,
            (LogLevel::Fatal, LogLevel::Fatal) => true,
            _ => false,
        }
    }

    /// Log to console
    fn log_to_console(&self, entry: &LogEntry) {
        let timestamp = chrono::DateTime::from_timestamp(entry.timestamp as i64, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S");

        let level_color = match entry.level {
            LogLevel::Trace => "\x1b[37m", // White
            LogLevel::Debug => "\x1b[36m", // Cyan
            LogLevel::Info => "\x1b[32m",  // Green
            LogLevel::Warn => "\x1b[33m",  // Yellow
            LogLevel::Error => "\x1b[31m", // Red
            LogLevel::Fatal => "\x1b[35m", // Magenta
        };

        let reset_color = "\x1b[0m";

        println!(
            "{}[{}] {}{} [{}] {}{}",
            level_color,
            timestamp,
            entry.level.as_str(),
            reset_color,
            entry.category.as_str(),
            entry.message,
            if let Some(ref details) = entry.details {
                format!(" - {}", details)
            } else {
                String::new()
            }
        );
    }

    /// Log to file
    fn log_to_file(&self, entry: &LogEntry) -> Result<(), QubeError> {
        let mut file_handle = self.file_handle.lock().unwrap();

        if let Some(ref mut file) = *file_handle {
            let log_line = if self.config.enable_json {
                serde_json::to_string(entry).unwrap()
            } else {
                format!(
                    "[{}] {} [{}] {} {}\n",
                    entry.timestamp,
                    entry.level.as_str(),
                    entry.category.as_str(),
                    entry.message,
                    entry.details.as_ref().unwrap_or(&String::new())
                )
            };

            file.write_all(log_line.as_bytes())
                .map_err(|e| QubeError::Io(e))?;
            file.flush().map_err(|e| QubeError::Io(e))?;
        }

        Ok(())
    }

    /// Update metrics
    fn update_metrics(&self, entry: &LogEntry) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.total_logs += 1;

        match entry.level {
            LogLevel::Trace => metrics.trace_count += 1,
            LogLevel::Debug => metrics.debug_count += 1,
            LogLevel::Info => metrics.info_count += 1,
            LogLevel::Warn => {
                metrics.warning_count += 1;
                metrics.last_warning = Some(entry.message.clone());
            }
            LogLevel::Error | LogLevel::Fatal => {
                metrics.error_count += 1;
                metrics.last_error = Some(entry.message.clone());
            }
        }
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> LogMetrics {
        self.metrics.lock().unwrap().clone()
    }

    /// Clear log file
    pub fn clear_logs(&self) -> Result<(), QubeError> {
        if self.config.enable_file {
            std::fs::remove_file(&self.config.log_file).map_err(|e| QubeError::Io(e))?;
            self.initialize_file()?;
        }
        Ok(())
    }

    /// Rotate log file if it's too large
    pub fn rotate_logs(&self) -> Result<(), QubeError> {
        if !self.config.enable_file {
            return Ok(());
        }

        let metadata = std::fs::metadata(&self.config.log_file).map_err(|e| QubeError::Io(e))?;

        if metadata.len() > self.config.max_file_size {
            // Create rotated filename
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let rotated_name = format!("{}.{}", self.config.log_file, timestamp);
            std::fs::rename(&self.config.log_file, &rotated_name).map_err(|e| QubeError::Io(e))?;

            // Reinitialize file
            self.initialize_file()?;
        }

        Ok(())
    }
}

/// Convenience functions for common logging operations
impl Logger {
    /// Log installation events
    pub fn log_installation(&self, message: &str, success: bool) -> Result<(), QubeError> {
        let level = if success {
            LogLevel::Info
        } else {
            LogLevel::Error
        };
        let entry = LogEntry::new(level, LogCategory::Installation, message.to_string());
        self.log(entry)
    }

    /// Log connection events
    pub fn log_connection(
        &self,
        message: &str,
        success: bool,
        user_id: Option<String>,
    ) -> Result<(), QubeError> {
        let level = if success {
            LogLevel::Info
        } else {
            LogLevel::Error
        };
        let mut entry = LogEntry::new(level, LogCategory::Connection, message.to_string());
        if let Some(user) = user_id {
            entry = entry.with_user(user);
        }
        self.log(entry)
    }

    /// Log query events
    pub fn log_query(&self, sql: &str, success: bool, duration_ms: u64) -> Result<(), QubeError> {
        let level = if success {
            LogLevel::Info
        } else {
            LogLevel::Error
        };
        let entry =
            LogEntry::new(level, LogCategory::Query, sql.to_string()).with_duration(duration_ms);
        self.log(entry)
    }

    /// Log database operations
    pub fn log_database(
        &self,
        operation: &str,
        success: bool,
        details: Option<String>,
    ) -> Result<(), QubeError> {
        let level = if success {
            LogLevel::Info
        } else {
            LogLevel::Error
        };
        let mut entry = LogEntry::new(level, LogCategory::Database, operation.to_string());
        if let Some(details) = details {
            entry = entry.with_details(details);
        }
        self.log(entry)
    }

    /// Log table operations
    pub fn log_table(
        &self,
        operation: &str,
        table_name: &str,
        success: bool,
    ) -> Result<(), QubeError> {
        let level = if success {
            LogLevel::Info
        } else {
            LogLevel::Error
        };
        let entry = LogEntry::new(level, LogCategory::Table, operation.to_string())
            .with_details(format!("Table: {}", table_name));
        self.log(entry)
    }

    /// Log index operations
    pub fn log_index(
        &self,
        operation: &str,
        index_name: &str,
        success: bool,
    ) -> Result<(), QubeError> {
        let level = if success {
            LogLevel::Info
        } else {
            LogLevel::Error
        };
        let entry = LogEntry::new(level, LogCategory::Index, operation.to_string())
            .with_details(format!("Index: {}", index_name));
        self.log(entry)
    }

    /// Log vector operations
    pub fn log_vector(
        &self,
        operation: &str,
        collection: &str,
        success: bool,
        duration_ms: u64,
    ) -> Result<(), QubeError> {
        let level = if success {
            LogLevel::Info
        } else {
            LogLevel::Error
        };
        let entry = LogEntry::new(level, LogCategory::Vector, operation.to_string())
            .with_details(format!("Collection: {}", collection))
            .with_duration(duration_ms);
        self.log(entry)
    }

    /// Log graph operations
    pub fn log_graph(
        &self,
        operation: &str,
        graph_name: &str,
        success: bool,
    ) -> Result<(), QubeError> {
        let level = if success {
            LogLevel::Info
        } else {
            LogLevel::Error
        };
        let entry = LogEntry::new(level, LogCategory::Graph, operation.to_string())
            .with_details(format!("Graph: {}", graph_name));
        self.log(entry)
    }

    /// Log performance metrics
    pub fn log_performance(
        &self,
        operation: &str,
        duration_ms: u64,
        memory_usage: u64,
        cpu_usage: f64,
    ) -> Result<(), QubeError> {
        let entry = LogEntry::new(
            LogLevel::Info,
            LogCategory::Performance,
            operation.to_string(),
        )
        .with_duration(duration_ms)
        .with_metrics(memory_usage, cpu_usage);
        self.log(entry)
    }

    /// Log error with details
    pub fn log_error(
        &self,
        category: LogCategory,
        message: &str,
        error: &QubeError,
        details: Option<String>,
    ) -> Result<(), QubeError> {
        let mut entry = LogEntry::new(LogLevel::Error, category, message.to_string())
            .with_error_code(format!("{:?}", error));

        if let Some(details) = details {
            entry = entry.with_details(details);
        }

        self.log(entry)
    }

    /// Log warning
    pub fn log_warning(
        &self,
        category: LogCategory,
        message: &str,
        details: Option<String>,
    ) -> Result<(), QubeError> {
        let mut entry = LogEntry::new(LogLevel::Warn, category, message.to_string());
        if let Some(details) = details {
            entry = entry.with_details(details);
        }
        self.log(entry)
    }

    /// Log info
    pub fn log_info(
        &self,
        category: LogCategory,
        message: &str,
        details: Option<String>,
    ) -> Result<(), QubeError> {
        let mut entry = LogEntry::new(LogLevel::Info, category, message.to_string());
        if let Some(details) = details {
            entry = entry.with_details(details);
        }
        self.log(entry)
    }
}

/// Global logger instance
static GLOBAL_LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

/// Initialize global logger
pub fn init_logger(config: LoggerConfig) -> Result<(), QubeError> {
    let logger = Logger::new(config)?;
    let mut global = GLOBAL_LOGGER.lock().unwrap();
    *global = Some(logger);
    Ok(())
}

/// Get global logger
pub fn get_logger() -> Option<&'static Logger> {
    let global = GLOBAL_LOGGER.lock().unwrap();
    global.as_ref().map(|l| unsafe { std::mem::transmute(l) })
}

/// Convenience functions for global logger
pub fn log_installation(message: &str, success: bool) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_installation(message, success)
    } else {
        Ok(())
    }
}

pub fn log_connection(
    message: &str,
    success: bool,
    user_id: Option<String>,
) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_connection(message, success, user_id)
    } else {
        Ok(())
    }
}

pub fn log_query(sql: &str, success: bool, duration_ms: u64) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_query(sql, success, duration_ms)
    } else {
        Ok(())
    }
}

pub fn log_database(
    operation: &str,
    success: bool,
    details: Option<String>,
) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_database(operation, success, details)
    } else {
        Ok(())
    }
}

pub fn log_table(operation: &str, table_name: &str, success: bool) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_table(operation, table_name, success)
    } else {
        Ok(())
    }
}

pub fn log_index(operation: &str, index_name: &str, success: bool) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_index(operation, index_name, success)
    } else {
        Ok(())
    }
}

pub fn log_vector(
    operation: &str,
    collection: &str,
    success: bool,
    duration_ms: u64,
) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_vector(operation, collection, success, duration_ms)
    } else {
        Ok(())
    }
}

pub fn log_graph(operation: &str, graph_name: &str, success: bool) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_graph(operation, graph_name, success)
    } else {
        Ok(())
    }
}

pub fn log_performance(
    operation: &str,
    duration_ms: u64,
    memory_usage: u64,
    cpu_usage: f64,
) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_performance(operation, duration_ms, memory_usage, cpu_usage)
    } else {
        Ok(())
    }
}

pub fn log_error(
    category: LogCategory,
    message: &str,
    error: &QubeError,
    details: Option<String>,
) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_error(category, message, error, details)
    } else {
        Ok(())
    }
}

pub fn log_warning(
    category: LogCategory,
    message: &str,
    details: Option<String>,
) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_warning(category, message, details)
    } else {
        Ok(())
    }
}

pub fn log_info(
    category: LogCategory,
    message: &str,
    details: Option<String>,
) -> Result<(), QubeError> {
    if let Some(logger) = get_logger() {
        logger.log_info(category, message, details)
    } else {
        Ok(())
    }
}
