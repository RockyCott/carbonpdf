//! Input source types for HTML content.

use std::path::PathBuf;
use crate::{Error, Result};

/// Source of HTML content to convert to PDF.
#[derive(Debug, Clone)]
pub enum InputSource {
    /// Raw HTML string
    Html(String),
    
    /// Path to local HTML file
    File(PathBuf),
    
    /// URL to fetch and render
    Url(String),
}

impl InputSource {
    /// Create an HTML string input source.
    pub fn html<S: Into<String>>(content: S) -> Self {
        Self::Html(content.into())
    }
    
    /// Create a file input source.
    pub fn file<P: Into<PathBuf>>(path: P) -> Self {
        Self::File(path.into())
    }
    
    /// Create a URL input source.
    pub fn url<S: Into<String>>(url: S) -> Self {
        Self::Url(url.into())
    }
    
    /// Resolve the input to an HTML string.
    pub async fn resolve(&self) -> Result<String> {
        match self {
            InputSource::Html(html) => Ok(html.clone()),
            
            InputSource::File(path) => {
                if !path.exists() {
                    return Err(Error::InputSource(
                        format!("File not found: {}", path.display())
                    ));
                }
                
                tokio::fs::read_to_string(path)
                    .await
                    .map_err(|e| Error::InputSource(
                        format!("Failed to read file {}: {}", path.display(), e)
                    ))
            }
            
            InputSource::Url(url) => {
                // For URLs, we don't fetch the content here - Chrome will navigate to it
                Ok(url.clone())
            }
        }
    }
}