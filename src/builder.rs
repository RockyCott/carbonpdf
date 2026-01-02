//! Fluent builder API for PDF generation.

use crate::{
    config::{ChromeConfig, Margins, Orientation, PageSize, PdfConfig},
    error::Result,
    input::InputSource,
    renderer::PdfRenderer,
};

#[cfg(feature = "chrome")]
use crate::renderer::chrome::ChromeRenderer;

/// Fluent builder for PDF generation.
///
/// # Examples
///
/// ```rust,no_run
/// use carbonpdf::{PdfBuilder, PageSize};
///
/// # #[tokio::main]
/// # async fn main() -> carbonpdf::Result<()> {
/// let pdf = PdfBuilder::new()
///     .html("<h1>Hello World</h1>")
///     .page_size(PageSize::A4)
///     .margin_all(1.0)
///     .print_background(true)
///     .build()
///     .await?;
/// # Ok(())
/// # }
/// ```

pub struct PdfBuilder {
    input: Option<InputSource>,
    config: PdfConfig,
    chrome_config: ChromeConfig,
}

impl Default for PdfBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfBuilder {
    /// Create a new PDF builder with default settings.
    pub fn new() -> Self {
        Self {
            input: None,
            config: PdfConfig::default(),
            chrome_config: ChromeConfig::default(),
        }
    }

    /// Set HTML content as input.
    pub fn html<S: Into<String>>(mut self, content: S) -> Self {
        self.input = Some(InputSource::html(content));
        self
    }

    /// Set file path as input.
    pub fn file<S: Into<std::path::PathBuf>>(mut self, path: S) -> Self {
        self.input = Some(InputSource::file(path));
        self
    }

    /// Set URL as input.
    pub fn url<S: Into<String>>(mut self, url: S) -> Self {
        self.input = Some(InputSource::url(url));
        self
    }

    /// Set page size.
    pub fn page_size(mut self, size: PageSize) -> Self {
        self.config.page_size = size;
        self
    }

    /// Set custom page size in inches.
    pub fn custom_page_size(mut self, width: f64, height: f64) -> Self {
        self.config.page_size = PageSize::Custom { width, height };
        self
    }

    /// Set page orientation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.config.orientation = orientation;
        self
    }

    /// Set all margins to the same value (in inches).
    pub fn margin_all(mut self, margin: f64) -> Self {
        self.config.margins = Margins::uniform(margin);
        self
    }

    /// Set individual margins (in inches).
    pub fn margins(mut self, top: f64, right: f64, bottom: f64, left: f64) -> Self {
        self.config.margins = Margins {
            top,
            bottom,
            left,
            right,
        };
        self
    }

    /// Set scale factor (0.1 to 2.0).
    pub fn scale(mut self, scale: f64) -> Self {
        self.config.scale = scale;
        self
    }

    /// Enable or disable background graphics printing.
    pub fn print_background(mut self, enabled: bool) -> Self {
        self.config.print_background = enabled;
        self
    }

    /// Set header template (HTML).
    pub fn header<S: Into<String>>(mut self, template: S) -> Self {
        self.config.header_template = Some(template.into());
        self.config.display_header_footer = true;
        self
    }

    /// Set footer template (HTML).
    pub fn footer<S: Into<String>>(mut self, template: S) -> Self {
        self.config.footer_template = Some(template.into());
        self.config.display_header_footer = true;
        self
    }

    /// Set generation timeout in seconds.
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.config.timeout_seconds = seconds;
        self
    }

    /// Set Chrome binary path.
    pub fn chrome_path<S: Into<String>>(mut self, path: S) -> Self {
        self.chrome_config.binary_path = Some(path.into());
        self
    }

    /// Disable Chrome sandbox (useful in Docker/CI).
    pub fn no_sandbox(mut self) -> Self {
        self.chrome_config.no_sandbox = true;
        self
    }

    /// Add custom Chrome arguments.
    pub fn chrome_args<I, S>(mut self, args: I) -> Self 
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.chrome_config
            .extra_args
            .extend(args.into_iter().map(|s| s.into()));
        self
    }

    /// Build and generate the PDF using Chrome renderer.
    #[cfg(feature = "chrome")]
    pub async fn build(self) -> Result<Vec<u8>> {
        let input = self
            .input
            .ok_or_else(|| crate::Error::InvalidConfig("No input source specified".to_string()))?;

        self.config.validate()?;

        let renderer = ChromeRenderer::new(self.chrome_config).await?;
        renderer.render(input, self.config).await
    }

    /// Build and generate the PDF using a custom renderer.
    pub async fn build_with<R: PdfRenderer>(
        self,
        renderer: &R,
    ) -> Result<Vec<u8>> {
        let input = self.input.ok_or_else(|| 
            crate::Error::InvalidConfig("No input source specified".to_string())
        )?;
        
        self.config.validate()?;
        
        renderer.render(input, self.config).await
    }
}
