//! # CarbonPDF
//!
//! Production-ready HTML to PDF conversion using Headless Chrome.
//!
//! CarbonPDF provides a safe, ergonomic API for converting HTML content to PDF documents
//! using Headless Chrome via the Chrome DevTools Protocol. It's designed for backend
//! services, CI/CD pipelines, and any application requiring high-quality PDF generation.
//!
//! ## Why Headless Chrome?
//!
//! - **Accurate rendering**: Chrome's rendering engine ensures PDFs match web output exactly
//! - **Modern web standards**: Full support for CSS3, JavaScript, web fonts, and flexbox/grid
//! - **Battle-tested**: Leverages the same engine used by billions of users daily
//!
//! ## Architecture
//!
//! CarbonPDF uses a layered architecture:
//! - **Public API**: Ergonomic builder pattern for common use cases
//! - **Renderer trait**: Abstraction allowing multiple backend implementations
//! - **Chrome backend**: Production-ready Chrome DevTools Protocol integration
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use carbonpdf::{PdfBuilder, PageSize, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Convert HTML string to PDF
//!     let pdf = PdfBuilder::new()
//!         .html("<h1>Hello, PDF!</h1>")
//!         .page_size(PageSize::A4)
//!         .margin_all(1.0)
//!         .build()
//!         .await?;
//!
//!     std::fs::write("output.pdf", pdf)?;
//!     Ok(())
//! }
//! ```
//!
//! ## Feature Flags
//!
//! - `chrome` (default): Enable Chrome backend via chromiumoxide
//! - `cli`: Build the `carbonpdf` CLI tool

#![warn(missing_docs, clippy::all)]
#![deny(unsafe_code)]

pub mod config;
pub mod error;
pub mod input;
pub mod builder;
pub mod renderer;
pub mod utils;

// Re-export commonly used types
pub use builder::PdfBuilder;
pub use config::{PdfConfig, PageSize, Orientation, Margins};
pub use error::{Error, Result};
pub use input::InputSource;
pub use renderer::PdfRenderer;
#[cfg(feature = "templates")]
pub mod template;

#[cfg(feature = "templates")]
pub use template::render_template;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "chrome")]
pub use renderer::chrome::ChromeRenderer;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");