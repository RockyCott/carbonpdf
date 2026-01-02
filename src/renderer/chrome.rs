//! Chrome/Chromium-based PDF renderer implementation.

use async_trait::async_trait;
use chromiumoxide::Browser;
use chromiumoxide::browser::BrowserConfig;
use chromiumoxide::cdp::browser_protocol::page::PrintToPdfParams;
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use tokio::task::JoinHandle;
use futures::StreamExt;
use tempfile::TempDir;

use crate::{
    config::{ChromeConfig, Orientation, PdfConfig},
    error::{Error, Result},
    input::InputSource,
    renderer::PdfRenderer,
};

/// Chrome-based PDF renderer.
///
/// This renderer uses Headless Chrome via the Chrome DevTools Protocol
/// to convert HTML to PDF with high fidelity.
///
/// # Examples
///
/// ```rust,no_run
/// use carbonpdf::{ChromeRenderer, InputSource, PdfConfig};
/// use carbonpdf::config::ChromeConfig;
/// use crate::carbonpdf::PdfRenderer;
///
/// # #[tokio::main]
/// # async fn main() -> carbonpdf::Result<()> {
/// let config = ChromeConfig::default();
/// let renderer = ChromeRenderer::new(config).await?;
///
/// let input = InputSource::html("<h1>Test</h1>");
/// let pdf_config = PdfConfig::default();
/// let pdf = renderer.render(input, pdf_config).await?;
/// # Ok(())
/// # }
/// ```
pub struct ChromeRenderer {
    browser: Arc<Browser>,
    _handler_task: JoinHandle<()>,
    _profile_dir: TempDir,
}

impl ChromeRenderer {
    /// Create a new Chrome renderer with the given configuration.
    pub async fn new(config: ChromeConfig) -> Result<Self> {
        let profile_dir = TempDir::new()
            .map_err(|e| Error::ChromeProcess(format!("Failed to create temp profile: {}", e)))?;

        let mut builder = BrowserConfig::builder()
            .user_data_dir(profile_dir.path());
        
        // Set Chrome binary path if provided
        if let Some(path) = config.binary_path {
            builder = builder.chrome_executable(path);
        }
        
        // Configure headless mode
        if config.headless {
            builder = builder.with_head();
        }
        
        // Add sandbox flag if requested
        if config.no_sandbox {
            builder = builder.arg("--no-sandbox");
        }
        
        // Add extra arguments
        for arg in config.extra_args {
            builder = builder.arg(arg);
        }
        
        let browser_config = builder.build()
            .map_err(|e| Error::ChromeProcess(format!("Failed to build config: {}", e)))?;
        
        // Launch browser with timeout
        let (browser, handler) = timeout(
            Duration::from_secs(config.connection_timeout),
            Browser::launch(browser_config)
        )
        .await
        .map_err(|_| Error::Timeout(config.connection_timeout))?
        .map_err(|e| Error::ChromeProcess(format!("Failed to launch Chrome: {}", e)))?;
        
        let handler_task = tokio::spawn(async move {
            let mut handler = handler;
            while let Some(event) = handler.next().await {
                if let Err(e) = event {
                    tracing::error!("Chrome handler error: {}", e);
                    break;
                }
            }
        });

        Ok(Self {
            browser: Arc::new(browser),
            _handler_task: handler_task,
            _profile_dir: profile_dir,
        })
    }
    
    /// Convert config to Chrome's PrintToPdfParams.
    fn build_print_params(config: &PdfConfig) -> PrintToPdfParams {
        let (width, height) = config.page_size.dimensions_inches();
        let (width, height) = match config.orientation {
            Orientation::Portrait => (width, height),
            Orientation::Landscape => (height, width),
        };
        
        PrintToPdfParams {
            landscape: matches!(config.orientation, Orientation::Landscape).into(),
            display_header_footer: Some(config.display_header_footer),
            print_background: Some(config.print_background),
            scale: Some(config.scale),
            paper_width: Some(width),
            paper_height: Some(height),
            margin_top: Some(config.margins.top),
            margin_bottom: Some(config.margins.bottom),
            margin_left: Some(config.margins.left),
            margin_right: Some(config.margins.right),
            page_ranges: None,
            header_template: config.header_template.clone(),
            footer_template: config.footer_template.clone(),
            generate_document_outline: Some(false),
            generate_tagged_pdf: Some(false),
            prefer_css_page_size: Some(config.prefer_css_page_size),
            transfer_mode: None,
        }
    }
}

#[async_trait]
impl PdfRenderer for ChromeRenderer {
    async fn render(&self, input: InputSource, config: PdfConfig) -> Result<Vec<u8>> {
        // Create a new page
        let page = self.browser.new_page("about:blank")
            .await
            .map_err(|e| Error::Protocol(format!("Failed to create page: {}", e)))?;
        
        // Load content based on input type
        match input {
            InputSource::Html(html) => {
                page.set_content(&html)
                    .await
                    .map_err(|e| Error::Protocol(format!("Failed to set content: {}", e)))?;
            }
            
            InputSource::File(path) => {
                let html = tokio::fs::read_to_string(&path)
                    .await
                    .map_err(|e| Error::InputSource(format!("Failed to read file: {}", e)))?;
                
                page.set_content(&html)
                    .await
                    .map_err(|e| Error::Protocol(format!("Failed to set content: {}", e)))?;
            }
            
            InputSource::Url(url) => {
                page.goto(&url)
                    .await
                    .map_err(|e| Error::Protocol(format!(
                        "Failed to navigate to URL '{}': {}",
                        url, e
                    )))?;

                page.wait_for_navigation()
                    .await
                    .map_err(|e| Error::Navigation(format!(
                        "Failed to navigate to URL '{}': {}",
                        url, e
                    )))?;
            }

        }
        
        // Generate PDF with timeout
        let print_params = Self::build_print_params(&config);
        
        let pdf_data = timeout(
            Duration::from_secs(config.timeout_seconds),
            page.pdf(print_params)
        )
        .await
        .map_err(|_| Error::Timeout(config.timeout_seconds))?
        .map_err(|e| Error::Protocol(format!("PDF generation failed: {}", e)))?;
        
        // Close the page to free resources
        let _ = page.close().await;
        
        Ok(pdf_data)
    }
    
    fn name(&self) -> &str {
        "ChromeRenderer"
    }
}

impl Drop for ChromeRenderer {
    fn drop(&mut self) {
        // Browser cleanup is handled by chromiumoxide
    }
}