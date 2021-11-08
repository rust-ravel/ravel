use crate::templates::HandlebarsTemplateEngine;
use actix_web::Error;
use actix_web::HttpResponse;
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
}

impl TemplateEngine {
    // forward render calls to each specific implementation.
    pub fn render(&self, view: &str, json: Value) -> Result<String, Error> {
        match self {
            #[cfg(feature = "handlebars")]
            TemplateEngine::HandlebarsEngine(template_engine) => template_engine.render(view, json),
            #[cfg(feature = "tera")]
            TemplateEngine::TeraEngine(template_engine) => template_engine.render(view, json),
            // TemplateEngine::None => Err(Error),
        }
    }

    pub fn response(&self, view: &str, json: Value) -> Result<HttpResponse, Error> {
        let html = self.render(view, json)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    }
}
