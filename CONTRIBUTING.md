# Contributing to Identikit

Thank you for your interest in contributing to Identikit! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## How to Contribute

### Reporting Issues

1. Check existing issues to avoid duplicates
2. Use the issue templates provided
3. Include clear reproduction steps for bugs
4. Provide system information (OS, Rust version, Node version)

### Pull Requests

1. Fork the repository
2. Create a feature branch from `main`
3. Follow the coding standards below
4. Write tests for new functionality
5. Update documentation as needed
6. Submit a pull request with a clear description

## Development Setup

### Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | 1.75+ | [rustup.rs](https://rustup.rs) |
| Bun | 1.0+ | [bun.sh](https://bun.sh) |
| Tauri CLI | 2.0+ | `cargo install tauri-cli` |

### Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/identikit.git
cd identikit

# Install frontend dependencies
cd frontend && bun install

# Run in development mode
bun run tauri dev
```

## Coding Standards

### Rust

- Follow standard Rust formatting (`cargo fmt`)
- Pass clippy checks (`cargo clippy -- -D warnings`)
- Document public APIs with doc comments
- Use meaningful variable and function names

### TypeScript/React

- Use TypeScript strict mode
- Follow ESLint configuration
- Use functional components with hooks
- Prefer named exports

### Commit Messages

Follow conventional commits format:

```
type(scope): description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat(canvas): add layer opacity control

Added opacity slider to adjustment panel.
Syncs with Fabric.js object opacity in real-time.
```

## Documentation

- Follow WHAT-WHY-HOW structure for technical docs
- Update relevant documentation when changing functionality
- Add JSDoc/rustdoc comments for public APIs

## Testing

### Backend (Rust)

```bash
cd backend
cargo test
```

### Frontend (React)

```bash
cd frontend
bun run test
```

### E2E Tests

```bash
cd frontend
bun run test:e2e
```

## Questions?

Open a discussion or reach out to the maintainers.

---

Thank you for contributing!
