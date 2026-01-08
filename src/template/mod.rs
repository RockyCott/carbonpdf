//! Template rendering module using Handlebars.

use crate::input::TemplateSource;
use crate::{Error, Result};
use handlebars::Handlebars;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::OnceCell;

/// Global Handlebars registry with helper functions.
static HANDLEBARS: OnceCell<Arc<Handlebars<'static>>> = OnceCell::const_new();

/// Initialize the Handlebars registry with built-in helpers.
async fn init_handlebars() -> Arc<Handlebars<'static>> {
    let mut hb = Handlebars::new();

    // Register built-in helpers
    hb.register_escape_fn(handlebars::html_escape);

    // Custom helper: format_currency
    hb.register_helper(
        "format_currency",
        Box::new(
            |h: &handlebars::Helper,
             _: &Handlebars,
             _: &handlebars::Context,
             _: &mut handlebars::RenderContext,
             out: &mut dyn handlebars::Output|
             -> handlebars::HelperResult {
                let value = h
                    .param(0)
                    .and_then(|v| v.value().as_f64())
                    .ok_or_else(|| handlebars::RenderError::new("Invalid currency value"))?;

                let symbol = h.param(1).and_then(|v| v.value().as_str()).unwrap_or("$");

                let formatted = format!("{}{:.2}", symbol, value);
                out.write(&formatted)?;
                Ok(())
            },
        ),
    );

    // Custom helper: format_date
    hb.register_helper(
        "format_date",
        Box::new(
            |h: &handlebars::Helper,
             _: &Handlebars,
             _: &handlebars::Context,
             _: &mut handlebars::RenderContext,
             out: &mut dyn handlebars::Output|
             -> handlebars::HelperResult {
                let date_str = h
                    .param(0)
                    .and_then(|v| v.value().as_str())
                    .ok_or_else(|| handlebars::RenderError::new("Invalid date value"))?;

                // Simple date formatting (in production, use chrono)
                out.write(date_str)?;
                Ok(())
            },
        ),
    );

    Arc::new(hb)
}

/// Get or initialize the global Handlebars registry.
async fn get_handlebars() -> Arc<Handlebars<'static>> {
    HANDLEBARS
        .get_or_init(|| async { init_handlebars().await })
        .await
        .clone()
}

/// Render a template with the provided data.
pub async fn render_template(source: &TemplateSource, data: &Value) -> Result<String> {
    let hb = get_handlebars().await;

    let template_content = match source {
        TemplateSource::Inline(template) => template.clone(),
        TemplateSource::File(path) => {
            if !path.exists() {
                return Err(Error::TemplateNotFound(path.display().to_string()));
            }

            tokio::fs::read_to_string(path)
                .await
                .map_err(|e| Error::Template(format!("Failed to read template file: {}", e)))?
        }
    };

    hb.render_template(&template_content, data)
        .map_err(|e| Error::Template(format!("Template rendering failed: {}", e)))
}

/// Register a custom helper function.
///
/// This allows users to add their own Handlebars helpers.
pub async fn register_helper<F>(name: &str, helper: F)
where
    F: handlebars::HelperDef + 'static,
{
    // Note: This requires making Handlebars mutable, which means
    // we'd need a different approach (RwLock) in production.
    // For now, this is a placeholder showing the API.
    let _ = (name, helper);
}
