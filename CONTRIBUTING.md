# Contributing to CarbonPDF

Thank you for your interest in contributing to CarbonPDF! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you agree to uphold this code.

## Getting Started

### Prerequisites

- Rust 1.91 or later
- Chrome or Chromium browser
- Git

### Setup

1. Fork the repository
2. Clone your fork:

```bash
git clone https://github.com/rockycRockyCottott/carbonpdf.git
cd carbonpdf
```

3. Build the project:

```bash
cargo build
```

4. Run tests:

```bash
   cargo test
```

## Development Workflow

### Making Changes

1. Create a feature branch:

```bash
   git checkout -b feature/my-feature
```

2. Make your changes following our coding standards
3. Add tests for new functionality
4. Ensure all tests pass:

```bash
   cargo test
   cargo clippy
   cargo fmt -- --check
```

5. Commit with clear messages:

```bash
   git commit -m "feat: add custom font support"
```

### Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions or changes
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Build/tooling changes

### Pull Request Process

1. Update documentation for any API changes
2. Add examples if introducing new features
3. Ensure CI passes
4. Request review from maintainers
5. Address feedback promptly

## Coding Standards

### Rust Style

- Follow the [Rust Style Guide](https://doc.rust-lang.org/beta/style-guide/)
- Run `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Maximum line length: 100 characters

### Documentation

- All public items must have doc comments
- Include examples in doc comments where helpful
- Doc examples must compile and run (`cargo test`)

Example:

```rust
/// Converts HTML to PDF.
///
/// # Arguments
///
/// * `html` - The HTML content to convert
///
/// # Examples
///
/// ```rust,no_run
/// # use carbonpdf::PdfBuilder;
/// # #[tokio::main]
/// # async fn main() -> carbonpdf::Result<()> {
/// let pdf = PdfBuilder::new()
///     .html("<h1>Test</h1>")
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub fn html<S: Into<String>>(self, content: S) -> Self {
    // ...
}
```

### Error Handling

- Use `Result<T, Error>` for fallible operations
- Provide context in error messages
- Use `thiserror` for error types
- Never `unwrap()` or `expect()` in library code

### Testing

- Unit tests in the same file: `#[cfg(test)] mod tests`
- Integration tests in `tests/` directory
- Use descriptive test names: `test_html_string_to_pdf`
- Test both success and error cases

## Architecture Guidelines

### Adding New Features

1. **Small, focused changes** - One feature per PR
2. **Maintain abstraction layers** - Keep separation between API/renderer/backend
3. **Preserve backward compatibility** - Follow semver strictly
4. **Document trade-offs** - Explain decisions in code comments

### Adding New Renderers

To add a new rendering backend:

1. Implement the `PdfRenderer` trait
2. Add as a feature flag in `Cargo.toml`
3. Update documentation with new backend info
4. Add integration tests

Example structure:

```rust
// src/renderer/playwright.rs

pub struct PlaywrightRenderer {
    // ...
}

#[async_trait]
impl PdfRenderer for PlaywrightRenderer {
    async fn render(&self, input: InputSource, config: PdfConfig) -> Result<Vec<u8>> {
        // Implementation
    }
    
    fn name(&self) -> &str {
        "PlaywrightRenderer"
    }
}
```

## Documentation

### README Updates

Update README.md when:

- Adding new features
- Changing API
- Adding installation steps
- Updating requirements

### API Documentation

Generate and review docs:

```bash
cargo doc --open
```

Ensure:

- All public items documented
- Examples compile
- Links work
- No warnings

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_html_string_to_pdf

# With logging
RUST_LOG=debug cargo test

# Integration tests only
cargo test --test integration_test
```

### Adding Tests

- Add unit tests for new logic
- Add integration tests for end-to-end flows
- Use fixtures in `tests/fixtures/` for test data
- Mock external dependencies where possible

## Performance

### Benchmarking

We use Criterion for benchmarks:

```bash
cargo bench
```

When optimizing:

1. Benchmark before changes
2. Make changes
3. Benchmark after
4. Document improvements in PR

### Performance PRs

Include in PR description:

- Benchmark results (before/after)
- Profiling data if applicable
- Trade-offs considered

## Questions?

- Open an issue for bug reports or feature requests
- Start a discussion for questions or ideas
- Join our community chat [link]

Thank you for contributing to CarbonPDF! ❤️
