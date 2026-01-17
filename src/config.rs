//! PDF configuration types.

use serde::{Deserialize, Serialize};

/// Complete PDF generation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfConfig {
    /// Page size configuration
    pub page_size: PageSize,
    
    /// Page orientation
    pub orientation: Orientation,
    
    /// Page margins in inches
    pub margins: Margins,
    
    /// Scale factor (0.1 to 2.0)
    pub scale: f64,
    
    /// Print background graphics
    pub print_background: bool,
    
    /// Display header and footer
    pub display_header_footer: bool,
    
    /// Header template (HTML)
    pub header_template: Option<String>,
    
    /// Footer template (HTML)
    pub footer_template: Option<String>,
    
    /// Prefer CSS page size
    pub prefer_css_page_size: bool,
    
    /// Whether to wait for all fonts to load before rendering
    pub wait_for_fonts: bool,
    
    /// Generation timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            page_size: PageSize::A4,
            orientation: Orientation::Portrait,
            margins: Margins::default(),
            scale: 1.0,
            print_background: true,
            display_header_footer: false,
            header_template: None,
            footer_template: None,
            prefer_css_page_size: false,
            wait_for_fonts: true,
            timeout_seconds: 30,
        }
    }
}

impl PdfConfig {
    /// Validate the configuration.
    pub fn validate(&self) -> crate::Result<()> {
        if !(0.1..=2.0).contains(&self.scale) {
            return Err(crate::Error::InvalidConfig(
                format!("Scale must be between 0.1 and 2.0, got {}", self.scale)
            ));
        }
        
        if self.timeout_seconds == 0 {
            return Err(crate::Error::InvalidConfig(
                "Timeout must be greater than 0".to_string()
            ));
        }
        
        Ok(())
    }
}

/// Standard page sizes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PageSize {
    /// A4 (210mm x 297mm)
    A4,
    /// Letter (8.5in x 11in)
    Letter,
    /// Legal (8.5in x 14in)
    Legal,
    /// Tabloid (11in x 17in)
    Tabloid,
    /// Custom size in inches (width, height)
    Custom {
        /// Width in inches
        width: f64,
        /// Height in inches
        height: f64,
    },
}

impl PageSize {
    /// Get dimensions in inches (width, height).
    pub fn dimensions_inches(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (8.27, 11.69),
            PageSize::Letter => (8.5, 11.0),
            PageSize::Legal => (8.5, 14.0),
            PageSize::Tabloid => (11.0, 17.0),
            PageSize::Custom { width, height } => (*width, *height),
        }
    }
}

/// Page orientation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Orientation {
    /// Portrait orientation
    Portrait,
    /// Landscape orientation
    Landscape,
}

/// Page margins in inches.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Margins {
    /// Top margin
    pub top: f64,
    /// Bottom margin
    pub bottom: f64,
    /// Left margin
    pub left: f64,
    /// Right margin
    pub right: f64,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            top: 0.4,
            bottom: 0.4,
            left: 0.4,
            right: 0.4,
        }
    }
}

impl Margins {
    /// Create uniform margins.
    pub fn uniform(margin: f64) -> Self {
        Self {
            top: margin,
            bottom: margin,
            left: margin,
            right: margin,
        }
    }
    
    /// Create margins with no spacing.
    pub fn none() -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }
}

/// Chrome browser configuration.
#[derive(Debug, Clone)]
pub struct ChromeConfig {
    /// Path to Chrome/Chromium binary
    pub binary_path: Option<String>,
    
    /// Disable sandbox (useful in Docker/CI)
    pub no_sandbox: bool,
    
    /// Additional Chrome flags
    pub extra_args: Vec<String>,
    
    /// Enable headless mode
    pub headless: bool,
    
    /// Connection timeout in seconds
    pub connection_timeout: u64,
}

impl Default for ChromeConfig {
    fn default() -> Self {
        Self {
            binary_path: None,
            no_sandbox: false,
            extra_args: vec![],
            headless: true,
            connection_timeout: 10,
        }
    }
}