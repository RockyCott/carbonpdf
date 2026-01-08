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

    /// Template source with data (requires `templates` feature)
    #[cfg(feature = "templates")]
    Template {
        /// Template source (inline or file)
        source: TemplateSource,
        /// Data to render the template with
        data: serde_json::Value,
    },
}

/// Template source for rendering.
#[cfg(feature = "templates")]
#[derive(Debug, Clone)]
pub enum TemplateSource {
    /// Inline template string
    Inline(String),
    
    /// Path to template file
    File(PathBuf),
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

    /// Create a template input source with inline template string.
    #[cfg(feature = "templates")]
    pub fn template<S, D>(template: S, data: D) -> Result<Self>
    where
        S: Into<String>,
        D: serde::Serialize,
    {
        let data = serde_json::to_value(data)
            .map_err(|e| Error::Template(format!("Failed to serialize data: {}", e)))?;
        
        Ok(Self::Template {
            source: TemplateSource::Inline(template.into()),
            data,
        })
    }

    /// Create a template input source with template file.
    #[cfg(feature = "templates")]
    pub fn template_file<P, D>(path: P, data: D) -> Result<Self>
    where
        P: Into<PathBuf>,
        D: serde::Serialize,
    {
        let data = serde_json::to_value(data)
            .map_err(|e| Error::Template(format!("Failed to serialize data: {}", e)))?;
        
        Ok(Self::Template {
            source: TemplateSource::File(path.into()),
            data,
        })
    }
    
    /// Resolve the input to an HTML string.
    ///
    /// For templates, this renders the template with the provided data.
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

            #[cfg(feature = "templates")]
            InputSource::Template { source, data } => {
                use crate::template::render_template;
                render_template(source, data).await
            }
        }
    }
}