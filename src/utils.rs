//! Utility functions for working with PDFs.

use crate::{Error, Result};
use base64::{engine::general_purpose, Engine as _};

/// Encode PDF bytes to base64 string.
///
/// Useful for embedding PDFs in JSON responses.
///
/// # Example
///
/// ```rust,no_run
/// use carbonpdf::{PdfBuilder, utils};
///
/// # #[tokio::main]
/// # async fn main() -> carbonpdf::Result<()> {
/// let pdf = PdfBuilder::new()
///     .html("<h1>Test</h1>")
///     .build()
///     .await?;
///
/// let base64 = utils::encode_base64(&pdf);
/// println!("data:application/pdf;base64,{}", base64);
/// # Ok(())
/// # }
/// ```
pub fn encode_base64(pdf: &[u8]) -> String {
    general_purpose::STANDARD.encode(pdf)
}

/// Decode base64 string to PDF bytes.
pub fn decode_base64(base64: &str) -> Result<Vec<u8>> {
    general_purpose::STANDARD
        .decode(base64)
        .map_err(|e| Error::Other(format!("Base64 decode error: {}", e)))
}

/// Get PDF metadata (basic validation).
///
/// Returns (version, page_count_estimate) or None if invalid.
pub fn get_pdf_info(pdf: &[u8]) -> Option<(String, usize)> {
    if !pdf.starts_with(b"%PDF-") {
        return None;
    }

    let version = String::from_utf8_lossy(&pdf[5..8]).to_string();
    let content = String::from_utf8_lossy(pdf);
    let page_count = content.matches("/Type /Page").count();

    Some((version, page_count))
}

/// Validate that bytes represent a valid PDF.
pub fn validate_pdf(pdf: &[u8]) -> Result<()> {
    if pdf.is_empty() || !pdf.starts_with(b"%PDF-") {
        return Err(Error::InvalidPdfData);
    }

    let content = String::from_utf8_lossy(pdf);
    if !content.contains("%%EOF") {
        return Err(Error::InvalidPdfData);
    }

    Ok(())
}
