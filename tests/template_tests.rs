#[cfg(all(test, feature = "templates"))]
mod template_tests {
    use carbonpdf::{
        template::render_template,
        InputSource, PdfBuilder, Result, TemplateSource,
    };
    use serde_json::json;

    #[tokio::test]
    async fn test_render_simple_template() -> Result<()> {
        let template = "<h1>Hello {{name}}!</h1>";
        let data = json!({"name": "World"});

        let source = TemplateSource::Inline(template.to_string());
        let html = render_template(&source, &data).await?;

        assert!(html.contains("Hello World!"));
        Ok(())
    }

    #[tokio::test]
    async fn test_template_input_source() -> Result<()> {
        let data = json!({"title": "Test"});
        let input = InputSource::template("<h1>{{title}}</h1>", data)?;

        let html = input.resolve().await?;
        assert!(html.contains("<h1>Test</h1>"));

        Ok(())
    }

    #[tokio::test]
    async fn test_custom_template_error() {
        let data = json!({"name": "Test"});
        let template = "{{invalid_helper name}}"; // Invalid helper

        let source = TemplateSource::Inline(template.to_string());
        let result = render_template(&source, &data).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Template"));
    }

    #[tokio::test]
    async fn test_template_with_currency_helper() -> Result<()> {
        let template = r#"
            <p>Price: {{format_currency 99.99 "$"}}</p>
        "#;

        let source = TemplateSource::Inline(template.to_string());
        let html = render_template(&source, &json!({})).await?;

        assert!(html.contains("$99.99"));
        Ok(())
    }
}