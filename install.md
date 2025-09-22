# 🚀 QubeDB Installation Guide

Panduan lengkap instalasi QubeDB di berbagai sistem operasi.

## 📋 Table of Contents

1. [Prerequisites](#prerequisites)
2. [Linux Installation](#linux-installation)
3. [Windows Installation](#windows-installation)
4. [macOS Installation](#macos-installation)
5. [Docker Installation](#docker-installation)
6. [Verification](#verification)
7. [Troubleshooting](#troubleshooting)

---

## 🔧 Prerequisites

### System Requirements

- **CPU**: x86_64 atau ARM64
- **RAM**: Minimum 2GB, Recommended 8GB+
- **Storage**: Minimum 1GB free space
- **Network**: Internet connection untuk download dependencies

### Required Software

- **Rust**: 1.90+ (untuk development)
- **Cargo**: Package manager Rust
- **Git**: Version control
- **Build Tools**: C/C++ compiler

---

## 🐧 Linux Installation

### Ubuntu/Debian

```bash
# 1. Update system
sudo apt update && sudo apt upgrade -y

# 2. Install dependencies
sudo apt install -y curl build-essential pkg-config libssl-dev

# 3. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 4. Verify Rust installation
rustc --version
cargo --version

# 5. Clone QubeDB repository
git clone https://github.com/qubedb/qubedb.git
cd qubedb/qubedb-core

# 6. Build QubeDB
cargo build --release

# 7. Run tests
cargo test

# 8. Install globally (optional)
sudo cp target/release/qubedb-core /usr/local/bin/qubedb
```

### CentOS/RHEL/Fedora

```bash
# 1. Update system
sudo dnf update -y  # or yum update -y

# 2. Install dependencies
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y curl pkg-config openssl-devel

# 3. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 4. Clone and build
git clone https://github.com/qubedb/qubedb.git
cd qubedb/qubedb-core
cargo build --release
```

### Arch Linux

```bash
# 1. Update system
sudo pacman -Syu

# 2. Install dependencies
sudo pacman -S base-devel curl pkg-config openssl

# 3. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 4. Clone and build
git clone https://github.com/qubedb/qubedb.git
cd qubedb/qubedb-core
cargo build --release
```

### Alpine Linux

```bash
# 1. Update system
apk update && apk upgrade

# 2. Install dependencies
apk add --no-cache curl build-base pkgconfig openssl-dev

# 3. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 4. Clone and build
git clone https://github.com/qubedb/qubedb.git
cd qubedb/qubedb-core
cargo build --release
```

---

## 🪟 Windows Installation

### Method 1: Using Rustup (Recommended)

```powershell
# 1. Download and install Rust
# Visit: https://rustup.rs/
# Or run in PowerShell:
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
.\rustup-init.exe

# 2. Restart PowerShell or run:
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# 3. Verify installation
rustc --version
cargo --version

# 4. Install Git (if not installed)
# Download from: https://git-scm.com/download/win

# 5. Clone repository
git clone https://github.com/qubedb/qubedb.git
cd qubedb\qubedb-core

# 6. Build QubeDB
cargo build --release

# 7. Run tests
cargo test
```

### Method 2: Using Chocolatey

```powershell
# 1. Install Chocolatey (if not installed)
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# 2. Install dependencies
choco install rust git -y

# 3. Clone and build
git clone https://github.com/qubedb/qubedb.git
cd qubedb\qubedb-core
cargo build --release
```

### Method 3: Using Scoop

```powershell
# 1. Install Scoop (if not installed)
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex

# 2. Install dependencies
scoop install rust git

# 3. Clone and build
git clone https://github.com/qubedb/qubedb.git
cd qubedb\qubedb-core
cargo build --release
```

### Visual Studio Build Tools

```powershell
# For advanced features (RocksDB, etc.)
# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/
# Select "C++ build tools" workload

# Or install via Chocolatey:
choco install visualstudio2019buildtools -y
```

---

## 🍎 macOS Installation

### Method 1: Using Homebrew (Recommended)

```bash
# 1. Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 2. Install dependencies
brew install rust git pkg-config openssl

# 3. Clone repository
git clone https://github.com/qubedb/qubedb.git
cd qubedb/qubedb-core

# 4. Build QubeDB
cargo build --release

# 5. Run tests
cargo test
```

### Method 2: Using MacPorts

```bash
# 1. Install MacPorts
# Download from: https://www.macports.org/install.php

# 2. Install dependencies
sudo port install rust git pkgconfig openssl

# 3. Clone and build
git clone https://github.com/qubedb/qubedb.git
cd qubedb/qubedb-core
cargo build --release
```

### Method 3: Manual Installation

```bash
# 1. Install Xcode Command Line Tools
xcode-select --install

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 3. Install Git
# Download from: https://git-scm.com/download/mac

# 4. Clone and build
git clone https://github.com/qubedb/qubedb.git
cd qubedb/qubedb-core
cargo build --release
```

---

## 🐳 Docker Installation

### Dockerfile

```dockerfile
# Dockerfile
FROM rust:1.90-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . /app
WORKDIR /app/qubedb-core

# Build QubeDB
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/qubedb-core/target/release/qubedb-core /usr/local/bin/qubedb

# Create data directory
RUN mkdir -p /data
VOLUME ["/data"]

# Expose port
EXPOSE 8080

# Run QubeDB
CMD ["qubedb", "--data-dir", "/data", "--port", "8080"]
```

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  qubedb:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - qubedb_data:/data
    environment:
      - QUBEDB_HOST=0.0.0.0
      - QUBEDB_PORT=8080
    restart: unless-stopped

volumes:
  qubedb_data:
```

### Build and Run

```bash
# Build Docker image
docker build -t qubedb .

# Run container
docker run -d \
  --name qubedb \
  -p 8080:8080 \
  -v qubedb_data:/data \
  qubedb

# Or use Docker Compose
docker-compose up -d
```

---

## ✅ Verification

### Basic Test

```bash
# Run basic example
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

### Performance Test

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

### Integration Test

```bash
# Test all drivers
cargo test --features "all-drivers"

# Test specific driver
cargo test --features "pdo-driver"
cargo test --features "django-driver"
cargo test --features "jdbc-driver"
```

---

## 🔧 Troubleshooting

### Common Issues

#### 1. Rust Installation Issues

```bash
# Problem: rustc not found
# Solution: Add to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Problem: Permission denied
# Solution: Fix permissions
sudo chown -R $USER:$USER ~/.cargo
```

#### 2. Build Errors

```bash
# Problem: missing libclang
# Solution: Install build tools
# Ubuntu/Debian:
sudo apt install -y build-essential libclang-dev

# CentOS/RHEL:
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y clang-devel

# Windows:
# Install Visual Studio Build Tools
```

#### 3. SSL/TLS Issues

```bash
# Problem: SSL certificate errors
# Solution: Update certificates
# Ubuntu/Debian:
sudo apt update && sudo apt install -y ca-certificates

# CentOS/RHEL:
sudo dnf update ca-certificates

# Or set environment variable:
export SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
```

#### 4. Memory Issues

```bash
# Problem: Out of memory during build
# Solution: Increase swap or reduce parallelism
export CARGO_BUILD_JOBS=1
cargo build --release

# Or add swap space:
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

#### 5. Network Issues

```bash
# Problem: Cannot download crates
# Solution: Use mirror or proxy
# Create ~/.cargo/config.toml:
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```

### Platform-Specific Issues

#### Windows

```powershell
# Problem: Microsoft Visual C++ not found
# Solution: Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/
# Select "C++ build tools" workload

# Problem: Git not found
# Solution: Install Git for Windows
# Download from: https://git-scm.com/download/win
```

#### macOS

```bash
# Problem: Xcode Command Line Tools not found
# Solution: Install Xcode Command Line Tools
xcode-select --install

# Problem: OpenSSL not found
# Solution: Install via Homebrew
brew install openssl
export PKG_CONFIG_PATH="/usr/local/opt/openssl/lib/pkgconfig"
```

#### Linux

```bash
# Problem: glibc version too old
# Solution: Update system or use older Rust version
rustup install 1.70.0
rustup default 1.70.0

# Problem: Missing system libraries
# Solution: Install development packages
# Ubuntu/Debian:
sudo apt install -y build-essential pkg-config libssl-dev

# CentOS/RHEL:
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y pkgconfig openssl-devel
```

### Getting Help

```bash
# Check Rust installation
rustc --version
cargo --version
rustup show

# Check system dependencies
pkg-config --version
openssl version

# Check QubeDB build
cargo check
cargo build --verbose

# Run with debug output
RUST_LOG=debug cargo run --example basic_usage
```

---

## 📞 Support

Jika mengalami masalah instalasi:
 
- 📧 **Email**: support@qubedb.com
- 💬 **Discord**: [Join our community](https://discord.gg/qubedb)
- 📖 **Documentation**: [docs.qubedb.com](https://docs.qubedb.com)
- 🐛 **Issues**: [GitHub Issues](https://github.com/qubedb/qubedb/issues)
- 📋 **Logs**: Kirim output `cargo build --verbose` untuk debugging

---

**QubeDB** - The future of databases is here! 🚀
