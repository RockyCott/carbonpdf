# CarbonPDF

Production-ready HTML to PDF conversion in Rust using Headless Chrome.

## Features

- **High-fidelity rendering** - Uses Headless Chrome for accurate HTML/CSS rendering
- **Production-ready** - Battle-tested architecture with comprehensive error handling
- **Fully configurable** - Control page size, margins, orientation, scale, and more
- **Easy to use** - Ergonomic builder API and sensible defaults
- **Type-safe** - Leverages Rust's type system for compile-time correctness
- **Async by default** - Built on Tokio for high-performance concurrent operations
- **Modern web standards** - Full support for CSS3, flexbox, grid, web fonts
- **Extensible** - Trait-based architecture for pluggable rendering backends

## Requirements

**Chrome or Chromium** must be installed and accessible. CarbonPDF will automatically detect Chrome in standard locations, or you can specify a custom path.

### Installation

On Ubuntu/Debian:

```bash
sudo apt-get install chromium-browser
```

On Arch Linux:

```bash
sudo pacman -S chromium
```

On macOS:

```bash
brew install --cask google-chrome
```

On Windows:
Download from [google.com/chrome](https://www.google.com/chrome)

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
carbonpdf = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Basic Usage

```rust
use carbonpdf::{PdfBuilder, PageSize, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Convert HTML string to PDF
    let pdf = PdfBuilder::new()
        .html("<h1>Hello, CarbonPDF!</h1><p>Easy PDF generation.</p>")
        .page_size(PageSize::A4)
        .margin_all(1.0)
        .build()
        .await?;

    std::fs::write("output.pdf", pdf)?;
    Ok(())
}
```

### From File

```rust
let pdf = PdfBuilder::new()
    .file("invoice.html")
    .page_size(PageSize::Letter)
    .build()
    .await?;
```

### From URL

```rust
let pdf = PdfBuilder::new()
    .url("https://example.com")
    .build()
    .await?;
```

## Template Features (Optional)

CarbonPDF includes optional Handlebars templating support for custom templates with variables.

### Enable Templates

Add the `templates` feature to your `Cargo.toml`:

```toml
[dependencies]
carbonpdf = { version = "0.2", features = ["templates"] }
```

### Custom Templates

Use your own Handlebars templates:

```rust
use carbonpdf::PdfBuilder;
use serde_json::json;

let template = r#"
    <h1>{{title}}</h1>
    <p>Hello {{name}}, your order #{{order_id}} is confirmed!</p>
    <ul>
    {{#each items}}
        <li>{{this}}</li>
    {{/each}}
    </ul>
"#;

let data = json!({
    "title": "Order Confirmation",
    "name": "Alice",
    "order_id": "12345",
    "items": ["Product A", "Product B", "Product C"]
});

let pdf = PdfBuilder::new()
    .template(template, data)?
    .build()
    .await?;
```

### Template Helpers

CarbonPDF includes custom Handlebars helpers:

- `{{format_currency value symbol}}` - Format currency values
- `{{format_date date_string}}` - Format dates

Example:

```handlebars
<p>Total: {{format_currency 1234.56 "$"}}</p>
<!-- Outputs: Total: $1234.56 -->
```

### Advanced Configuration

```rust
use carbonpdf::{PdfBuilder, PageSize, Orientation};

let pdf = PdfBuilder::new()
    .html(html_content)
    .page_size(PageSize::A4)
    .orientation(Orientation::Landscape)
    .margins(0.5, 0.75, 0.5, 0.75)  // top, right, bottom, left (inches)
    .scale(0.9)
    .print_background(true)
    .header("<div style='font-size: 10px;'>Page <span class='pageNumber'></span></div>")
    .footer("<div style='text-align: center;'>© 2024 Your Company</div>")
    .timeout(60)  // seconds
    .build()
    .await?;
```

### Custom Page Size

```rust
let pdf = PdfBuilder::new()
    .html(content)
    .custom_page_size(8.5, 14.0)  // width, height in inches
    .build()
    .await?;
```

### Docker/CI Configuration

```rust
let pdf = PdfBuilder::new()
    .html(content)
    .no_sandbox()  // Required in Docker
    .chrome_args(["--disable-dev-shm-usage"])
    .build()
    .await?;
```

## CLI Tool

Install the CLI:

```bash
cargo install carbonpdf --features cli
```

Usage:

```bash
# Convert HTML file
carbonpdf input.html -o output.pdf

# Convert from URL
carbonpdf https://example.com -o example.pdf

# With options
carbonpdf input.html -o output.pdf \
    --page-size A4 \
    --landscape \
    --margin 1.0 \
    --scale 0.9
```

## Architecture

CarbonPDF uses a layered architecture:

1. **Public API Layer** - Builder pattern and high-level types
2. **Renderer Abstraction** - `PdfRenderer` trait for backend implementations
3. **Chrome Backend** - Chrome DevTools Protocol integration
4. **Process Management** - Automatic Chrome lifecycle handling

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design documentation.

## Error Handling

CarbonPDF provides detailed error types:

```rust
use carbonpdf::Error;

match pdf_result {
    Ok(pdf) => println!("Success!"),
    Err(Error::ChromeProcess(msg)) => eprintln!("Chrome error: {}", msg),
    Err(Error::Timeout(secs)) => eprintln!("Timed out after {}s", secs),
    Err(Error::InvalidConfig(msg)) => eprintln!("Config error: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Best Practices

### In Backend Services

```rust
// Reuse renderer instance across requests
lazy_static! {
    static ref RENDERER: ChromeRenderer = {
        ChromeRenderer::new(ChromeConfig::default())
            .await
            .expect("Failed to initialize Chrome")
    };
}

async fn generate_invoice(html: String) -> Result<Vec<u8>> {
    RENDERER.render(
        InputSource::html(html),
        PdfConfig::default()
    ).await
}
```

### In CI/CD

```dockerfile
# Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y \
    chromium \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/myapp /usr/local/bin/
CMD ["myapp"]
```

```rust
// In your code
let pdf = PdfBuilder::new()
    .html(content)
    .no_sandbox()  // Required in Docker
    .chrome_args(["--disable-dev-shm-usage"])
    .build()
    .await?;
```

## Performance Tips

- **Reuse renderer instances** - Browser initialization is expensive
- **Set appropriate timeouts** - Default is 30s, adjust based on content complexity
- **Optimize HTML** - Minimize external resources, inline critical CSS
- **Use connection pooling** - For high-volume scenarios (coming in v0.2)

## Roadmap

- [ ] Automatic Chrome download (via feature flag)
- [ ] Connection pooling for high-volume scenarios
- [ ] Playwright backend support
- [ ] PDF/A compliance mode
- [ ] Watermarking support
- [ ] Parallel batch processing utilities
- [ ] Add templates to cli
- [ ] Add stdin to cli
- [ ] Add --css to cli
- [ ] migrar a subcommands (carbonpdf render)

## Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

Built on the shoulders of giants:

- [chromiumoxide](https://github.com/mattsse/chromiumoxide) - Chrome DevTools Protocol
- [tokio](https://tokio.rs) - Async runtime
- The Chromium team for an amazing rendering engine
