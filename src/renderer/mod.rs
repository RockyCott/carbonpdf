//! PDF rendering abstraction and implementations.

use async_trait::async_trait;
use crate::{config::PdfConfig, Result};

#[cfg(feature = "chrome")]
pub mod chrome;

/// Resolved input for rendering.
#[derive(Debug, Clone)]
pub enum ResolvedInput {
    /// Raw HTML string
    Html(String),
    /// URL to fetch and render
    Url(String),
}

/// Trait for PDF rendering backends.
///
/// This abstraction allows multiple rendering implementations (Chrome, Playwright, etc.)
#[async_trait]
pub trait PdfRenderer: Send + Sync {
    /// Render HTML content to PDF.
    ///
    /// # Arguments
    ///
    /// * `input` - Source of HTML content (string, file, or URL)
    /// * `config` - PDF generation configuration
    ///
    /// # Returns
    ///
    /// PDF file as a byte vector
    async fn render(&self, input: ResolvedInput, config: PdfConfig) -> Result<Vec<u8>>;
    
    /// Get the renderer name (for logging/debugging).
    fn name(&self) -> &str;
}