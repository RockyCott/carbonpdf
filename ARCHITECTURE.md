# CarbonPDF Architecture

## Design Philosophy

CarbonPDF is designed with the following principles:

1. **Separation of Concerns** - Clear boundaries between API, rendering, and Chrome interaction
2. **Extensibility** - Trait-based design allows alternative rendering backends
3. **Safety** - No unsafe code, comprehensive error handling, resource cleanup
4. **Ergonomics** - Builder pattern API with sensible defaults
5. **Performance** - Async-first design, efficient resource management

## System Architecture

### Layer 1: Public API

The public API is the only surface users interact with:

- **PdfBuilder** - Fluent builder for configuration
- **InputSource** - Type-safe input variants (HTML/File/URL)
- **PdfConfig** - Complete PDF generation configuration
- **ChromeConfig** - Browser-specific settings

Design decisions:

- Builder pattern for discoverability and flexibility
- Strong typing prevents invalid configurations
- Validation at build time, not runtime

### Layer 2: Renderer Abstraction

The `PdfRenderer` trait decouples the API from implementation:

```rust
#[async_trait]
pub trait PdfRenderer: Send + Sync {
    async fn render(&self, input: InputSource, config: PdfConfig) -> Result<Vec<u8>>;
    fn name(&self) -> &str;
}
```

**Why?**

- Allows future backends (Playwright, wkhtmltopdf, etc.)
- Simplifies testing with mock implementations
- Clear contracts and boundaries

### Layer 3: Chrome Backend

The Chrome backend consists of:

1. **ChromeRenderer** - Implements PdfRenderer
2. **Process Management** - Browser lifecycle
3. **Protocol Layer** - CDP communication

Key responsibilities:

- Launch and manage Chrome process
- Translate PdfConfig to CDP parameters
- Handle errors and timeouts
- Resource cleanup

## Chrome Lifecycle Management

### Initialization

```text
┌─────────────────┐
│ ChromeRenderer  │
│    ::new()      │
└────────┬────────┘
│
▼
┌─────────────────┐
│ Launch Chrome   │
│ with flags      │
└────────┬────────┘
│
▼
┌─────────────────┐
│ Connect to CDP  │
│ via WebSocket   │
└────────┬────────┘
│
▼
┌─────────────────┐
│ Return renderer │
└─────────────────┘
```

### Per-Request Flow

1. Create new browser page (tab)
2. Load content (set_content or goto)
3. Wait for network idle
4. Call Page.printToPDF
5. Receive PDF data
6. Close page (cleanup)

**Why create new pages per request?**

- Isolation between requests
- Parallel processing capability
- Simpler error recovery

### Shutdown

The browser is closed when ChromeRenderer is dropped. Chromiumoxide handles:

- Closing all pages
- Terminating Chrome process
- Cleaning up temporary files

## Error Handling Strategy

CarbonPDF uses typed errors with context:

```rust
pub enum Error {
    ChromeProcess(String),  // Launch/crash
    Protocol(String),        // CDP communication
    InvalidConfig(String),   // User configuration
    InputSource(String),     // File/URL issues
    Network(reqwest::Error), // HTTP errors
    Timeout(u64),            // Timeout exceeded
    // ...
}
```

**Error handling principles:**

- Fail fast with clear messages
- Provide actionable guidance
- Never panic in library code
- Clean up resources on error

## Configuration Design

### Two-tier configuration

1. **PdfConfig** - Document properties (size, margins, etc.)
   - Portable across renderers
   - Validated before rendering

2. **ChromeConfig** - Browser-specific settings
   - Separate from document config
   - Chrome-specific optimizations

This separation ensures:

- Clear ownership of concerns
- Easy to add new renderers
- Testable without Chrome

## Thread Safety

CarbonPDF is designed to be thread-safe:

- `PdfRenderer` requires `Send + Sync`
- `ChromeRenderer` uses `Arc<Browser>`
- No shared mutable state
- Async API is Send-safe

Safe usage pattern:

```rust
lazy_static! {
    static ref RENDERER: ChromeRenderer = /* ... */;
}

// Safe to call from multiple threads/tasks
tokio::spawn(async {
    RENDERER.render(/* ... */).await
});
```

## Resource Management

### Memory

- PDFs are returned as `Vec<u8>` (owned)
- Large PDFs may require streaming (future enhancement)
- Browser pages closed immediately after use

### Process

- Chrome launched once per renderer instance
- Pages (tabs) created per request
- Automatic cleanup on Drop

### Files

- Temporary files handled by chromiumoxide
- No file artifacts left behind
- Input files read asynchronously

## Testing Strategy

### Unit Tests

- Configuration validation
- Error construction
- Input source resolution

### Integration Tests

- End-to-end PDF generation
- Various configurations
- Error scenarios

### Test Isolation

- Tests don't require Chrome (where possible)
- Mock implementations of PdfRenderer
- Fixtures for deterministic tests

## Trade-offs and Limitations

### Current Design

**Pros:**

- Simple, predictable API
- Full Chrome rendering capabilities
- Type-safe configuration
- Clear error messages

**Cons:**

- Chrome dependency (large binary)
- Process overhead per renderer
- No streaming output (yet)
- Single backend currently

### Known Limitations

1. **Chrome Required** - Not embeddable library
   - Mitigation: Clear error messages, auto-download feature (roadmap)

2. **Process Overhead** - Chrome is heavy
   - Mitigation: Reuse renderer instances, connection pooling (roadmap)

3. **No Streaming** - Full PDF in memory
   - Mitigation: Reasonable for most use cases, streaming planned

4. **Synchronous Drop** - Cleanup in drop
   - Mitigation: Works fine, async drop would be ideal but not available

## Future Enhancements

### v0.2

- Connection pooling for high-volume scenarios
- Streaming PDF output
- Automatic Chrome download

### v0.3

- Playwright backend support
- PDF/A compliance mode
- Batch processing utilities

## Security Considerations

1. **No user code execution** - HTML is rendered, not executed as Rust code
2. **Sandbox mode** - Chrome runs sandboxed by default (disable only in trusted environments)
3. **Input validation** - All config validated before processing
4. **No unsafe code** - Pure safe Rust (except in dependencies)
5. **Resource limits** - Timeouts prevent infinite hangs

## Performance Characteristics

- **Browser startup**: ~1-2 seconds (one-time cost)
- **Page creation**: ~50-100ms per page
- **PDF generation**: Depends on content complexity
- **Memory**: ~100MB base + content size

**Optimization recommendations:**

1. Reuse ChromeRenderer instances
2. Keep HTML simple, inline critical CSS
3. Set appropriate timeouts
4. Consider connection pooling for high-volume

