//! Use a custom Handlebars template with your own data structure.
//!
//! Run with: cargo run --example custom_template --features templates

#[cfg(feature = "templates")]
use carbonpdf::{PdfBuilder, Result};

#[cfg(feature = "templates")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "templates")]
#[derive(Debug, Serialize, Deserialize)]
struct Certificate {
    recipient_name: String,
    course_name: String,
    completion_date: String,
    instructor_name: String,
    hours: u32,
}

#[cfg(feature = "templates")]
#[tokio::main]
async fn main() -> Result<()> {
    // Custom certificate template
    let template = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {
                    font-family: 'Georgia', serif;
                    text-align: center;
                    padding: 100px 80px;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                }
                
                .certificate {
                    background: white;
                    padding: 60px;
                    border-radius: 20px;
                    box-shadow: 0 20px 60px rgba(0,0,0,0.3);
                    border: 10px solid #ffd700;
                }
                
                h1 {
                    font-size: 48px;
                    color: #667eea;
                    margin-bottom: 20px;
                    text-transform: uppercase;
                    letter-spacing: 3px;
                }
                
                .subtitle {
                    font-size: 20px;
                    color: #666;
                    margin-bottom: 40px;
                }
                
                .recipient {
                    font-size: 36px;
                    color: #333;
                    font-weight: bold;
                    margin: 30px 0;
                    border-bottom: 3px solid #667eea;
                    padding-bottom: 10px;
                    display: inline-block;
                }
                
                .course {
                    font-size: 24px;
                    color: #764ba2;
                    margin: 30px 0;
                }
                
                .details {
                    font-size: 16px;
                    color: #666;
                    margin-top: 40px;
                }
                
                .signature {
                    margin-top: 60px;
                    display: inline-block;
                    border-top: 2px solid #333;
                    padding-top: 10px;
                    font-size: 18px;
                }
            </style>
        </head>
        <body>
            <div class="certificate">
                <h1>Certificate of Completion</h1>
                <div class="subtitle">This is to certify that</div>
                
                <div class="recipient">{{recipient_name}}</div>
                
                <div class="subtitle">has successfully completed</div>
                
                <div class="course">{{course_name}}</div>
                
                <div class="details">
                    {{hours}} hours of instruction<br>
                    Completed on {{completion_date}}
                </div>
                
                <div class="signature">
                    {{instructor_name}}<br>
                    <small>Course Instructor</small>
                </div>
            </div>
        </body>
        </html>
    "#;

    // Create certificate data
    let certificate = Certificate {
        recipient_name: "JB Sevani".to_string(),
        course_name: "Advanced Rust Programming".to_string(),
        completion_date: "January 15, 2025".to_string(),
        instructor_name: "Dr. JB Sevani".to_string(),
        hours: 40,
    };

    println!("Generating certificate for: {}", certificate.recipient_name);

    // Generate PDF with custom template
    let pdf = PdfBuilder::new()
        .template(template, certificate)?
        .page_size(carbonpdf::PageSize::Letter)
        .orientation(carbonpdf::Orientation::Landscape)
        .margin_all(0.0) // No margins for certificate
        .print_background(true)
        .build()
        .await?;

    let filename = "certificate.pdf";
    std::fs::write(filename, pdf)?;

    println!("Certificate PDF generated: {}", filename);

    Ok(())
}

#[cfg(not(feature = "templates"))]
fn main() {
    println!("This example requires the 'templates' feature.");
    println!("Run with: cargo run --example custom_template --features templates");
}