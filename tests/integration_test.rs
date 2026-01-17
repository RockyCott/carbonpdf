//! Integration tests for CarbonPDF.

use carbonpdf::{Orientation, PageSize, PdfBuilder, Result};
use std::path::Path;

#[tokio::test]
async fn test_html_string_to_pdf() -> Result<()> {
    let pdf = PdfBuilder::new()
        .html("<h1>Test Document</h1><p>This is a test.</p>")
        .page_size(PageSize::A4)
        .build()
        .await?;

    assert!(!pdf.is_empty());
    assert!(pdf.starts_with(b"%PDF")); // PDF magic number

    Ok(())
}

#[tokio::test]
async fn test_landscape_orientation() -> Result<()> {
    let pdf = PdfBuilder::new()
        .html("<h1>Landscape Document</h1>")
        .orientation(Orientation::Landscape)
        .build()
        .await?;

    assert!(!pdf.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_custom_margins() -> Result<()> {
    let pdf = PdfBuilder::new()
        .html("<h1>Custom Margins</h1>")
        .margins(0.5, 0.75, 0.5, 0.75)
        .build()
        .await?;

    assert!(!pdf.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_file_input() -> Result<()> {
    let test_file = Path::new("tests/fixtures/sample.html");

    if test_file.exists() {
        let pdf = PdfBuilder::new().file(test_file).build().await?;

        assert!(!pdf.is_empty());
    }
    
    Ok(())
}

#[tokio::test]
async fn test_scale_factor() -> Result<()> {
    let pdf = PdfBuilder::new()
        .html("<h1>Scaled Document</h1>")
        .scale(0.8)
        .build()
        .await?;

    assert!(!pdf.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_wait_for_fonts() -> Result<()> {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <link href="https://fonts.googleapis.com/css2?family=Roboto&display=swap" rel="stylesheet">
            <style>body { font-family: 'Roboto', sans-serif; }</style>
        </head>
        <body>
            <h1>Font Test</h1>
        </body>
        </html>
    "#;

    let pdf = PdfBuilder::new()
        .html(html)
        .wait_for_fonts(true)
        .build()
        .await?;

    assert!(!pdf.is_empty());
    assert!(pdf.starts_with(b"%PDF"));

    Ok(())
}

#[tokio::test]
#[should_panic(expected = "No input source specified")]
async fn test_missing_input() {
    let _ = PdfBuilder::new()
        .page_size(PageSize::A4)
        .build()
        .await
        .unwrap();
}
