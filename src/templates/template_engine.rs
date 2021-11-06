use crate::templates::HandlebarsTemplateEngine;
use crate::templates::TemplateError;
use axum::response::Html;
use serde_json::Value;

/// Render a html template depending on the backend
/// enabled by the feature flags. This creates a template engine.
#[derive(Debug, Clone)]
pub enum TemplateEngine {
    /// Create a handlebars template renderer.
    #[cfg(feature = "handlebars")]
    HandlebarsEngine(HandlebarsTemplateEngine),
    /// Create a tera template renderer.
    #[cfg(feature = "tera")]
    TeraEngine(TeraTemplateEngine),
    /// No template engine configured through feature flags.
    None,
}

impl TemplateEngine {
    // forward render calls to each specific implementation.
    pub fn render(&self, view: &str, json: Value) -> Result<String, TemplateError> {
        match self {
            #[cfg(feature = "handlebars")]
            TemplateEngine::HandlebarsEngine(template_engine) => template_engine.render(view, json),
            #[cfg(feature = "tera")]
            TemplateEngine::TeraEngine(template_engine) => template_engine.render(view, json),
            TemplateEngine::None => Err(TemplateError::Registration("Tried to render template without setting template engine. Use feature: <'handlebars'|'tera'>.".to_string())),
        }
    }

    pub fn response(&self, view: &str, json: Value) -> Result<Html<String>, TemplateError> {
        let html = self.render(view, json)?;
        Ok(Html::from(html))
    }
}
