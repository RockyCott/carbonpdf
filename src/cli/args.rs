use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use crate::PageSize;

#[derive(Parser, Debug)]
#[command(name = "carbonpdf")]
#[command(about = "Convert HTML to PDF using Headless Chrome")]
#[command(version)]
pub struct Cli {
    /// Input file (HTML) or URL
    pub input: String,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Page size
    #[arg(long, value_enum, default_value_t = PageSizeArg::A4)]
    pub page_size: PageSizeArg,

    /// Custom page width in inches (requires --page-size custom)
    #[arg(long, requires = "custom_height")]
    pub custom_width: Option<f64>,

    /// Custom page height in inches (requires --page-size custom)
    #[arg(long, requires = "custom_width")]
    pub custom_height: Option<f64>,

    /// Use landscape orientation
    #[arg(long)]
    pub landscape: bool,

    /// Margin in inches (applied to all sides)
    #[arg(short, long)]
    pub margin: Option<f64>,

    #[arg(long)]
    pub margin_top: Option<f64>,
    #[arg(long)]
    pub margin_bottom: Option<f64>,
    #[arg(long)]
    pub margin_left: Option<f64>,
    #[arg(long)]
    pub margin_right: Option<f64>,

    /// Scale factor (0.1–2.0)
    #[arg(long, default_value_t = 1.0)]
    pub scale: f64,

    /// Disable background graphics
    #[arg(long)]
    pub no_background: bool,

    /// Path to Chrome/Chromium binary
    #[arg(long)]
    pub chrome_path: Option<String>,

    /// Disable Chrome sandbox (for Docker/CI)
    #[arg(long)]
    pub no_sandbox: bool,

    /// Timeout in seconds
    #[arg(long, default_value_t = 30)]
    pub timeout: u64,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PageSizeArg {
    A4,
    Letter,
    Legal,
    Tabloid,
    Custom,
}

impl From<PageSizeArg> for PageSize {
    fn from(arg: PageSizeArg) -> Self {
        match arg {
            PageSizeArg::A4 => PageSize::A4,
            PageSizeArg::Letter => PageSize::Letter,
            PageSizeArg::Legal => PageSize::Legal,
            PageSizeArg::Tabloid => PageSize::Tabloid,
            PageSizeArg::Custom => PageSize::A4,
        }
    }
}
