# 🖥️ QubeDB Desktop GUI

Modern desktop application for QubeDB database management, built with Tauri and Rust.

## ✨ Features

- 🎨 **Modern UI** - Clean, intuitive interface
- 🔌 **Database Connections** - Easy connection management
- 📝 **SQL Editor** - Syntax highlighting and formatting
- 📊 **Query Results** - Table view with export options
- 🗄️ **Database Browser** - Tree view of databases and tables
- ⚡ **Real-time** - Live query execution
- 🔐 **Security** - Secure connection handling
- 🚀 **Performance** - Built with Rust for speed

## 🚀 Quick Start

### Prerequisites

- Windows 10/11
- Rust 1.90+ (for development)
- Node.js 16+ (for development)

### Installation

#### Option 1: Download Installer
1. Download `QubeDB-Setup.exe` from releases
2. Run the installer
3. Follow the setup wizard
4. Launch QubeDB Desktop from desktop shortcut

#### Option 2: Build from Source
```bash
# Clone repository
git clone https://github.com/qubedb/qubedb.git
cd qubedb/qubedb-gui

# Install dependencies
npm install

# Build application
cargo build --release

# Run application
cargo run
```

## 🎯 Usage

### 1. Connect to Database
- Click "New Connection" button
- Fill in connection details:
  - **Name**: Connection name
  - **Host**: localhost (default)
  - **Port**: 8080 (default)
  - **Database**: Database name
  - **Username**: admin (default)
  - **Password**: Your password
- Click "Connect"

### 2. Execute Queries
- Write SQL queries in the editor
- Use `Ctrl+Enter` to execute
- View results in the results panel
- Export results if needed

### 3. Browse Database
- Use the database tree in sidebar
- Click on tables to view structure
- Right-click for context menu options

## 🎨 Screenshots

### Main Interface
```
┌─────────────────────────────────────────────────────────┐
│ QubeDB Desktop                                          │
├─────────────┬───────────────────────────────────────────┤
│ Connections │ SQL Query Editor                          │
│ • My DB     │ ┌─────────────────────────────────────────┐ │
│             │ │ SELECT * FROM users WHERE age > 18;    │ │
│ Databases   │ │                                         │ │
│ • database  │ │                                         │ │
│   • tables  │ │                                         │ │
│   • views   │ └─────────────────────────────────────────┘ │
│             │                                           │
│             │ Results                                   │
│             │ ┌─────────────────────────────────────────┐ │
│             │ │ id │ name    │ email           │ age   │ │
│             │ │ 1  │ John    │ john@example.com│ 25    │ │
│             │ │ 2  │ Jane    │ jane@example.com│ 30    │ │
│             │ └─────────────────────────────────────────┘ │
└─────────────┴───────────────────────────────────────────┘
```

## 🔧 Development

### Project Structure
```
qubedb-gui/
├── src/                    # Rust backend
│   ├── main.rs            # Main application
│   └── lib.rs             # Library code
├── src-tauri/             # Tauri configuration
│   └── tauri.conf.json    # Tauri config
├── dist/                  # Frontend assets
│   ├── index.html         # Main HTML
│   ├── styles.css         # CSS styles
│   └── script.js          # JavaScript
└── Cargo.toml             # Rust dependencies
```

### Building
```bash
# Development build
cargo run

# Release build
cargo build --release

# Build installer
cargo tauri build
```

### Adding Features
1. Add Rust functions in `src/main.rs`
2. Expose via Tauri commands
3. Call from JavaScript in `dist/script.js`
4. Update UI in `dist/index.html`

## 🎨 Customization

### Themes
The GUI supports custom themes. Create CSS files in `dist/themes/`:

```css
/* Dark theme example */
body {
    background: #1a1a1a;
    color: #ffffff;
}

.sidebar {
    background: #2d2d2d;
}
```

### Plugins
Extend functionality with plugins:

```rust
// Add custom commands
#[tauri::command]
async fn custom_function() -> Result<String, String> {
    // Your custom logic
    Ok("Hello from plugin!".to_string())
}
```

## 🔐 Security

- **Connection Security**: All connections use TLS by default
- **Password Storage**: Passwords are encrypted in memory
- **SQL Injection**: Parameterized queries prevent injection
- **File Access**: Limited to application directory

## 🐛 Troubleshooting

### Common Issues

#### 1. Application Won't Start
```bash
# Check Rust installation
rustc --version

# Rebuild application
cargo clean
cargo build --release
```

#### 2. Connection Failed
- Check if QubeDB server is running
- Verify connection parameters
- Check firewall settings
- Ensure port 8080 is available

#### 3. GUI Not Loading
```bash
# Clear cache
cargo clean

# Rebuild with verbose output
cargo run --verbose
```

### Debug Mode
```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run with GUI debug
cargo run -- --debug
```

## 📖 API Reference

### Tauri Commands

#### Database Operations
```rust
// Connect to database
connect_database(connection: DatabaseConnection) -> String

// Execute query
execute_query(request: QueryRequest) -> QueryResponse

// Get tables
get_tables(connection_id: String) -> Vec<String>
```

#### File Operations
```rust
// Save query
save_query(query: String) -> Result<(), String>

// Load query
load_query(filename: String) -> String

// Export results
export_results(data: QueryResult, format: String) -> String
```

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- 📧 **Email**: support@qubedb.com
- 💬 **Discord**: [Join our community](https://discord.gg/qubedb)
- 📖 **Documentation**: [docs.qubedb.com](https://docs.qubedb.com)
- 🐛 **Issues**: [GitHub Issues](https://github.com/qubedb/qubedb/issues)

---

**QubeDB Desktop** - The future of database management! 🚀
