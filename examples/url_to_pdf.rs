//! Convert a URL to PDF.

use carbonpdf::{PageSize, PdfBuilder, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://www.rust-lang.org".to_string());

    println!("Converting {} to PDF...", url);

    let pdf = PdfBuilder::new()
        .url(url)
        .page_size(PageSize::A4)
        .print_background(true)
        .timeout(60)
        .build()
        .await?;

    let filename = "url_output.pdf";
    std::fs::write(filename, pdf)?;

    println!("PDF generated: {}", filename);
    println!(" Size: {} bytes", std::fs::metadata(filename)?.len());

    Ok(())
}
