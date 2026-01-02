/// Example with advanced configuration options.
use carbonpdf::{Orientation, PageSize, PdfBuilder, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                @page {
                    size: A4;
                    margin: 0;
                }
                body {
                    font-family: 'Helvetica', sans-serif;
                    margin: 0;
                    padding: 40px;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                }
                .container {
                    background: white;
                    padding: 40px;
                    border-radius: 12px;
                    box-shadow: 0 10px 40px rgba(0,0,0,0.1);
                }
                h1 {
                    color: #667eea;
                    margin: 0 0 20px 0;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Advanced PDF Configuration</h1>
                <p>This PDF demonstrates:</p>
                <ul>
                    <li>Custom page size and orientation</li>
                    <li>Background graphics printing</li>
                    <li>Custom margins and scale</li>
                    <li>Header and footer templates</li>
                </ul>
            </div>
        </body>
        </html>
    "#;

    let pdf = PdfBuilder::new()
        .html(html)
        .page_size(PageSize::A4)
        .orientation(Orientation::Portrait)
        .margins(0.75, 0.5, 0.75, 0.5)
        .scale(0.95)
        .print_background(true)
        .header(
            r#"
            <div style="font-size: 10px; text-align: right; width: 100%; padding: 10px;">
                <strong>Company Report</strong> | Page <span class="pageNumber"></span>
            </div>
        "#,
        )
        .footer(
            r#"
            <div style="font-size: 9px; text-align: center; width: 100%; color: #666;">
                Confidential - Generated on <span class="date"></span>
            </div>
        "#,
        )
        .timeout(45)
        .build()
        .await?;

    std::fs::write("advanced_output.pdf", pdf)?;
    println!("Advanced PDF generated: advanced_output.pdf");

    Ok(())
}
