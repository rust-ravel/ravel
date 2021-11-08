use crate::templates::TemplateEngine;
// use crate::templates::TemplateError;
use crate::templates::TemplateOptions;
use actix_web::error::ErrorBadRequest;
use actix_web::Error;
use handlebars_::Handlebars;
// use handlebars_::TemplateError as HandlebarsTemplateError;
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct HandlebarsTemplateEngine {
    pub(crate) engine: Handlebars<'static>,
}

impl HandlebarsTemplateEngine {
    pub fn register(params: TemplateOptions) -> TemplateEngine {
        let mut handlebars = Handlebars::new();
        let path = Path::new(params.get_directory());

        handlebars
            .register_templates_directory(".hbs", path)
            .expect("Failed to register template. TODO: better error messaging.");

        TemplateEngine::HandlebarsEngine(Self { engine: handlebars })
    }

    pub fn render(&self, view: &str, json: Value) -> Result<String, Error> {
        match self.engine.render(view, &json) {
            Ok(string) => Ok(string),
            Err(render_error) => Err(ErrorBadRequest(render_error)),
        }
    }
}
