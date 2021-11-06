use crate::templates::TemplateEngine;
use crate::templates::TemplateError;
use crate::templates::TemplateOptions;
use handlebars_::Handlebars;
// use handlebars_::TemplateError as HandlebarsTemplateError;
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct HandlebarsTemplateEngine {
    pub(crate) engine: Handlebars<'static>,
}

impl HandlebarsTemplateEngine {
    pub fn register(params: TemplateOptions) -> Result<TemplateEngine, TemplateError> {
        let mut handlebars = Handlebars::new();
        let path = Path::new(params.get_directory());

        match handlebars.register_templates_directory(".hbs", path) {
            Ok(()) => Ok(TemplateEngine::HandlebarsEngine(Self {
                engine: handlebars,
            })),
            Err(template_error) => Err(TemplateError::Registration(String::from(
                "Failed to register template. TODO: better error messaging.",
            ))),
        }
    }

    pub fn render(&self, view: &str, json: Value) -> Result<String, TemplateError> {
        match self.engine.render(view, &json) {
            Ok(string) => Ok(string),
            Err(render_error) => Err(TemplateError::Registration(render_error.desc)),
        }
    }
}
