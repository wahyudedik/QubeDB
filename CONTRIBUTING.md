# Contributing to QubeDB

Thank you for your interest in contributing to QubeDB! This document provides guidelines and information for contributors.

## 🤝 How to Contribute

### Reporting Issues

Before creating an issue, please:
1. Check if the issue already exists
2. Use the issue templates
3. Provide detailed information about the problem

### Suggesting Enhancements

We welcome enhancement suggestions! Please:
1. Check if the enhancement is already planned
2. Provide a clear description of the proposed enhancement
3. Explain the benefits and use cases

### Code Contributions

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## 🛠️ Development Setup

### Prerequisites

- Rust 1.70+
- Cargo
- Git
- Docker (optional, for testing)

### Getting Started

```bash
# Fork and clone the repository
git clone https://github.com/your-username/qubedb.git
cd qubedb

# Install dependencies
cd qubedb-core
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Project Structure

```
qubedb/
├── qubedb-core/          # Core database engine
│   ├── src/
│   │   ├── storage/      # Storage engine
│   │   ├── query/        # Query engine
│   │   ├── drivers/      # Language drivers
│   │   └── lib.rs
│   └── Cargo.toml
├── qubedb-gui/           # Web GUI
├── docs/                 # Documentation
├── examples/             # Examples
└── tests/                # Integration tests
```

## 📝 Coding Standards

### Rust Code Style

We follow Rust's official style guidelines:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings
```

### Code Organization

- Use meaningful variable and function names
- Add documentation for public APIs
- Write tests for new functionality
- Keep functions small and focused
- Use appropriate error handling

### Documentation

- Document all public APIs
- Use rustdoc comments
- Provide examples in documentation
- Keep documentation up to date

## 🧪 Testing

### Unit Tests

```bash
# Run unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration_tests

# Run all tests
cargo test --all
```

### Benchmarks

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench benchmark_name
```

## 📋 Pull Request Process

### Before Submitting

1. **Fork and clone** the repository
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes**
4. **Add tests** for new functionality
5. **Run tests** and ensure they pass
6. **Update documentation** if needed
7. **Commit your changes** (`git commit -m 'Add amazing feature'`)
8. **Push to your branch** (`git push origin feature/amazing-feature`)

### Pull Request Guidelines

1. **Use descriptive titles** and descriptions
2. **Reference related issues** using `#issue_number`
3. **Add screenshots** for UI changes
4. **Include test results** and benchmarks
5. **Request reviews** from relevant team members

### Review Process

1. **Automated checks** must pass
2. **Code review** by maintainers
3. **Testing** on multiple platforms
4. **Documentation** review
5. **Final approval** and merge

## 🏷️ Issue Labels

We use labels to categorize issues:

- `bug` - Something isn't working
- `enhancement` - New feature or request
- `documentation` - Improvements to documentation
- `good first issue` - Good for newcomers
- `help wanted` - Extra attention is needed
- `priority: high` - High priority
- `priority: medium` - Medium priority
- `priority: low` - Low priority

## 🎯 Development Roadmap

See our [Master Roadmap](QubeDB-Master-Roadmap.md) for the complete development plan.

### Current Focus Areas

- **Core Engine**: WAL, BTree, LSM implementation
- **SQL Layer**: Parser, optimizer, executor
- **Networking**: HTTP, gRPC, Postgres wire protocol
- **Client Libraries**: Multi-language SDKs
- **Production Features**: HA, monitoring, security

## 💬 Communication

### Getting Help

- **GitHub Discussions**: Ask questions and get help
- **Discord**: Join our community server
- **Email**: contact@qubedb.com

### Code of Conduct

We are committed to providing a welcoming and inclusive experience for everyone. Please:

- Be respectful and inclusive
- Use welcoming and inclusive language
- Be respectful of differing viewpoints
- Accept constructive criticism gracefully
- Focus on what's best for the community

## 📄 License

By contributing to QubeDB, you agree that your contributions will be licensed under the MIT License.

## 🙏 Recognition

Contributors will be recognized in:
- **CONTRIBUTORS.md** file
- **Release notes** for significant contributions
- **GitHub contributors** page
- **Community highlights**

## 📞 Contact

- **GitHub**: [@qubedb](https://github.com/qubedb)
- **Discord**: [QubeDB Community](https://discord.gg/qubedb)
- **Email**: contributors@qubedb.com
- **Twitter**: [@QubeDB](https://twitter.com/qubedb)

---

Thank you for contributing to QubeDB! 🚀
