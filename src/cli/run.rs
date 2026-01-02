use std::path::PathBuf;
use tracing::info;
use clap::Parser;

use crate::{Error, Orientation, PdfBuilder, Result};

use super::args::{Cli, PageSizeArg};
use super::logging;

/// Run the CLI application.
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    logging::init(cli.verbose);

    info!("Starting CarbonPDF CLI");

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let pdf = build_pdf(&cli).await?;
        std::fs::write(&cli.output, &pdf)?;
        print_summary(&cli.output, pdf.len());
        Ok(())
    })
}

async fn build_pdf(cli: &Cli) -> Result<Vec<u8>> {
    let mut builder = PdfBuilder::new();

    builder = apply_input(builder, cli)?;
    builder = apply_page_size(builder, cli)?;
    builder = apply_orientation(builder, cli);
    builder = apply_margins(builder, cli);
    builder = apply_render_options(builder, cli);
    builder = apply_chrome_config(builder, cli);

    info!("Generating PDF...");
    builder.build().await
}

fn apply_input(builder: PdfBuilder, cli: &Cli) -> Result<PdfBuilder> {
    if cli.input.starts_with("http://") || cli.input.starts_with("https://") {
        info!("Input detected as URL");
        Ok(builder.url(&cli.input))
    } else {
        let path = PathBuf::from(&cli.input);
        if path.exists() {
            info!("Input detected as file");
            Ok(builder.file(path))
        } else {
            Err(Error::InputSource(format!(
                "Input file not found: {}",
                cli.input
            )))
        }
    }
}

fn apply_page_size(builder: PdfBuilder, cli: &Cli) -> Result<PdfBuilder> {
    match cli.page_size {
        PageSizeArg::Custom => {
            let (w, h) = cli.custom_width.zip(cli.custom_height).ok_or_else(|| {
                Error::InvalidConfig(
                    "Custom page size requires --custom-width and --custom-height".into(),
                )
            })?;
            info!("Using custom page size: {}x{} inches", w, h);
            Ok(builder.custom_page_size(w, h))
        }
        _ => {
            info!("Using page size: {:?}", cli.page_size);
            Ok(builder.page_size(cli.page_size.into()))
        }
    }
}

fn apply_orientation(mut builder: PdfBuilder, cli: &Cli) -> PdfBuilder {
    if cli.landscape {
        info!("Using landscape orientation");
        builder = builder.orientation(Orientation::Landscape);
    }
    builder
}

fn apply_margins(builder: PdfBuilder, cli: &Cli) -> PdfBuilder {
    if let Some(margin) = cli.margin {
        info!("Using uniform margin: {} inches", margin);
        builder.margin_all(margin)
    } else {
        let top = cli.margin_top.unwrap_or(0.4);
        let bottom = cli.margin_bottom.unwrap_or(0.4);
        let left = cli.margin_left.unwrap_or(0.4);
        let right = cli.margin_right.unwrap_or(0.4);

        info!("Using margins T:{} R:{} B:{} L:{}", top, right, bottom, left);
        builder.margins(top, right, bottom, left)
    }
}

fn apply_render_options(builder: PdfBuilder, cli: &Cli) -> PdfBuilder {
    builder
        .scale(cli.scale)
        .print_background(!cli.no_background)
        .timeout(cli.timeout)
}

fn apply_chrome_config(mut builder: PdfBuilder, cli: &Cli) -> PdfBuilder {
    if let Some(path) = &cli.chrome_path {
        builder = builder.chrome_path(path.clone());
    }
    if cli.no_sandbox {
        info!("Chrome sandbox disabled");
        builder = builder.no_sandbox();
    }
    builder
}

fn print_summary(path: &PathBuf, size: usize) {
    println!("PDF successfully generated");
    println!(" Output: {}", path.display());
    println!(" Size: {} bytes ({:.2} KB)", size, size as f64 / 1024.0);
}
