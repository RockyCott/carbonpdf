//! PDF rendering abstraction and implementations.

use async_trait::async_trait;
use crate::{config::PdfConfig, input::InputSource, Result};

#[cfg(feature = "chrome")]
pub mod chrome;

/// Trait for PDF rendering backends.
///
/// This abstraction allows multiple rendering implementations (Chrome, Playwright, etc.)
/// while maintaining a consistent API.
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
    async fn render(&self, input: InputSource, config: PdfConfig) -> Result<Vec<u8>>;
    
    /// Get the renderer name (for logging/debugging).
    fn name(&self) -> &str;
}