//! Error types for CarbonPDF operations.

use thiserror::Error;

/// Result type alias using CarbonPDF's error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during PDF generation.
#[derive(Error, Debug)]
pub enum Error {
    /// Chrome binary not found or failed to launch
    #[error("Chrome process error: {0}")]
    ChromeProcess(String),

    /// Chrome DevTools Protocol communication error
    #[error("Chrome DevTools Protocol error: {0}")]
    Protocol(String),

    /// Invalid configuration provided
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Input source error (file not found, invalid URL, etc.)
    #[error("Input source error: {0}")]
    InputSource(String),

    /// Template rendering error
    #[cfg(feature = "templates")]
    #[error("Template error: {0}")]
    Template(String),

    /// Template not found
    #[cfg(feature = "templates")]
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    /// Navigation error when loading pages
    #[error("Navigation error: {0}")]
    Navigation(String),

    /// Network error when fetching URLs
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// PDF generation timeout
    #[error("PDF generation timed out after {0} seconds")]
    Timeout(u64),

    /// Chrome returned invalid PDF data
    #[error("Invalid PDF data returned from Chrome")]
    InvalidPdfData,

    /// Other errors
    #[error("Unexpected error: {0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Other(s.to_string())
    }
}