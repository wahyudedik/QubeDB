# 🚀 QubeDB Setup Guide - Install & Test di Komputer

Panduan lengkap untuk install dan test QubeDB di komputer Windows kamu!

## 📋 Prerequisites (Yang Diperlukan)

### 1. **Rust (Wajib)**
```bash
# Download dan install Rust
# Kunjungi: https://rustup.rs/
# Atau run di PowerShell:
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
.\rustup-init.exe

# Restart PowerShell atau run:
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# Verify installation
rustc --version
cargo --version
```

### 2. **Git (Wajib)** 
```bash
# Download dari: https://git-scm.com/download/win
# Atau install via Chocolatey:
choco install git -y
```

### 3. **Visual Studio Build Tools (Optional - untuk fitur advanced)**
```bash
# Download dari: https://visualstudio.microsoft.com/downloads/
# Select "C++ build tools" workload
# Atau install via Chocolatey:
choco install visualstudio2019buildtools -y
```

## 🚀 **Method 1: Build dari Source (Recommended)**

### Step 1: Clone Repository
```bash
# Clone QubeDB
git clone https://github.com/wahyudedik/QubeDB.git
cd QubeDB
```

### Step 2: Build QubeDB Core
```bash
# Masuk ke directory core
cd qubedb-core

# Build release version
cargo build --release

# Test build
cargo test
```

### Step 3: Test Basic Functionality
```bash
# Run basic example
cargo run --example basic_usage

# Run performance test
cargo run --example performance_test
```

### Step 4: Build GUI (Optional)
```bash
# Kembali ke root directory
cd ..

# Masuk ke GUI directory
cd qubedb-gui

# Install Tauri dependencies (jika belum)
npm install

# Build GUI
cargo build --release

# Run GUI
cargo run
```

## 🚀 **Method 2: Quick Test (Tanpa Build)**

### Step 1: Download Pre-built (Jika Ada)
```bash
# Download dari GitHub Releases
# https://github.com/wahyudedik/QubeDB/releases
```

### Step 2: Run Examples
```bash
# Test basic functionality
cd qubedb-core
cargo run --example basic_usage
```

## 🧪 **Testing QubeDB**

### Test 1: Basic Database Operations
```bash
# Run basic usage example
cargo run --example basic_usage

# Expected output:
# 🦀 QubeDB Core - Next Generation Multi-Model Database
# ==================================================
# 
# 📦 Testing Storage Engine...
# ✅ Storage engine initialized
# 
# 🔍 Testing Query Engine...
# ✅ Query engine initialized
# 
# 📝 Testing SQL Parser...
# ✅ SQL parsed successfully
# 
# 💾 Testing Data Storage...
# ✅ Data stored successfully
# 
# 🔍 Testing Data Retrieval...
# ✅ Data retrieved: {...}
# 
# 🧠 Testing Vector Storage...
# ✅ Vector stored successfully
# ✅ Vector retrieved: [0.1, 0.2, 0.3, 0.4, 0.5]
# 
# 🎉 All tests completed successfully!
```

### Test 2: Performance Test
```bash
# Run performance benchmark
cargo run --example performance_test

# Expected output:
# ⚡ QubeDB Performance Test
# =========================
# 
# 📊 Insert Performance:
# - 100 records: 634.3µs
# - 1,000 records: 5.2ms
# - 10,000 records: 45.8ms
# 
# 🔍 Query Performance:
# - Simple SELECT: 0.1ms
# - Complex JOIN: 2.3ms
# - Vector Search: 1.8ms
# 
# 🧠 Memory Usage:
# - Base memory: 2.1MB
# - After 10K records: 15.3MB
# - Memory efficiency: 95.2%
```

### Test 3: GUI Test (Jika Build GUI)
```bash
# Run GUI application
cargo run

# Expected: GUI window opens dengan interface management
```

## 🔧 **Troubleshooting**

### Problem 1: Rust Not Found
```bash
# Solution: Add Rust to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or on Windows:
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
```

### Problem 2: Build Errors
```bash
# Solution: Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Problem 3: Missing Dependencies
```bash
# Solution: Install build tools
# Windows:
choco install visualstudio2019buildtools -y

# Or download Visual Studio Build Tools manually
```

### Problem 4: GUI Not Loading
```bash
# Solution: Install Node.js
# Download from: https://nodejs.org/
# Or via Chocolatey:
choco install nodejs -y

# Then rebuild GUI
cd qubedb-gui
npm install
cargo build --release
```

## 🎯 **Quick Start Commands**

### 1. **Clone & Build**
```bash
git clone https://github.com/wahyudedik/QubeDB.git
cd QubeDB/qubedb-core
cargo build --release
```

### 2. **Test Core**
```bash
cargo run --example basic_usage
cargo run --example performance_test
```

### 3. **Test GUI (Optional)**
```bash
cd ../qubedb-gui
cargo build --release
cargo run
```

### 4. **Run All Tests**
```bash
cd ../qubedb-core
cargo test
```

## 📊 **Expected Results**

### ✅ **Success Indicators:**
- ✅ Rust installation: `rustc --version` shows version
- ✅ Cargo installation: `cargo --version` shows version
- ✅ Build success: No compilation errors
- ✅ Tests pass: All tests show "✅"
- ✅ Performance: Sub-millisecond response times
- ✅ GUI opens: Desktop window appears (if built)

### ❌ **Common Issues:**
- ❌ "rustc not found" → Install Rust
- ❌ "cargo not found" → Add to PATH
- ❌ Build errors → Update Rust, install build tools
- ❌ GUI not loading → Install Node.js, rebuild

## 🚀 **Next Steps Setelah Install**

### 1. **Explore Features**
- Test different data types (String, Int, Float, JSON, Vector)
- Try SQL queries (SELECT, INSERT, UPDATE, DELETE)
- Test vector similarity search
- Test graph operations

### 2. **Integration Test**
- Test dengan Laravel project
- Test dengan Django project
- Test dengan Spring Boot project
- Test dengan Node.js project

### 3. **Performance Test**
- Run benchmark dengan data besar
- Test concurrent operations
- Monitor memory usage
- Test dengan real-world scenarios

## 📞 **Jika Ada Masalah**

### Debug Commands:
```bash
# Check Rust version
rustc --version
cargo --version

# Check build
cargo check

# Verbose build
cargo build --verbose

# Run with debug
RUST_LOG=debug cargo run --example basic_usage
```

### Get Help:
- 📧 **Email**: support@qubedb.com
- 💬 **Discord**: [Join our community](https://discord.gg/qubedb)
- 📖 **Documentation**: [docs.qubedb.com](https://docs.qubedb.com)
- 🐛 **Issues**: [GitHub Issues](https://github.com/qubedb/qubedb/issues)

---

**Selamat mencoba QubeDB! 🚀**

Jika ada masalah, jangan ragu untuk tanya! 😄
