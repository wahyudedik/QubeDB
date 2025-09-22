# 🦀 QubeDB - Next Generation Multi-Model Database

[![Rust](https://img.shields.io/badge/rust-1.90+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Performance](https://img.shields.io/badge/performance-200K%2B%20records%2Fs-brightgreen.svg)](https://github.com/yourusername/QubeDB)

> **QubeDB** adalah database multi-model modern yang ditulis dalam Rust yang menggabungkan kekuatan database relational, document, graph, dan vector dalam satu sistem yang terpadu.

## ✨ Features

- 🗄️ **Multi-Model Support** - Relational, Document, Graph, Vector dalam satu database
- 🚀 **High Performance** - 200K+ records/second, 100K+ vectors/second
- 🦀 **Rust Powered** - Memory safe, fast, dan reliable
- 📦 **Embedded Mode** - Seperti SQLite, bisa embedded di aplikasi
- 🔌 **Multiple Drivers** - Laravel, Django, Spring, Node.js, Go, Rust
- 🧠 **AI Ready** - Vector search untuk aplikasi AI/ML
- ☁️ **Cloud Native** - Siap untuk deployment cloud dan edge

## 🚀 Quick Start

```bash
# Clone repository
git clone https://github.com/yourusername/QubeDB.git
cd QubeDB/qubedb-core

# Build
cargo build --release

# Run example
cargo run --example basic_usage

# Run performance test
cargo run --example performance_test
```

## 📊 Performance

| Operation | Performance |
|-----------|-------------|
| Insert | 200K+ records/second |
| Vector Search | 100K+ vectors/second |
| Query | Sub-millisecond response |
| Memory | Efficient Rust management |

## 📖 Documentation

- [Installation Guide](install.md)
- [Performance Testing](test.md)
- [Framework Integration](docs.md)
- [Complete Documentation](dokumentasi.md)
- [Development Roadmap](roadmap.md)

## 🏗️ Architecture

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
                    │   Network Layer │
                    │                 │
                    │ • gRPC          │
                    │ • REST API      │
                    │ • GraphQL       │
                    └─────────────────┘
```

## 🎯 Use Cases

- **E-commerce** - Product catalog, user management, recommendations
- **AI/ML Applications** - Vector search, embeddings, similarity
- **Social Networks** - User relationships, content, graph traversal
- **IoT Systems** - Time-series data, real-time analytics
- **Enterprise** - Multi-model data, complex queries

## 🤝 Contributing

Kontribusi sangat diterima! Silakan lihat [Contributing Guide](CONTRIBUTING.md) untuk detail.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🚀 Roadmap

- [x] **Tahun 1** - Fundamental & Riset ✅
- [x] **Tahun 2** - MVP (Minimum Viable Product) ✅
- [ ] **Tahun 3** - Stabil & Cloud-Ready
- [ ] **Tahun 4** - Monetisasi & Scale
- [ ] **Tahun 5** - Ekspansi Global

## 📞 Support

- 📧 **Email**: support@qubedb.com
- 💬 **Discord**: [Join our community](https://discord.gg/qubedb)
- 📖 **Documentation**: [docs.qubedb.com](https://docs.qubedb.com)
- 🐛 **Issues**: [GitHub Issues](https://github.com/yourusername/QubeDB/issues)

---

**QubeDB** - The future of databases is here! 🚀