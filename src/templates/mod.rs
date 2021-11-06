// use axum::response::Html;
// // #[cfg(feature = "handlebars")]
// // use handlebars_::Handlebars;
// use serde_json::Value;
// use std::path::Path;
use driver::*;
use error::TemplateError;

pub use template_engine::TemplateEngine;

pub mod driver;
mod error;
mod template_engine;

/// Defines a template
#[derive(Debug, Default)]
pub struct Template;

/// Defines the configuration options of a database
#[derive(Debug)]
pub struct TemplateOptions {
    /// The directory to register template files in
    pub(crate) directory: String,
}

impl Template {
    /// Method to create a [DatabaseConnection] on a database
    pub fn register<C>(opt: C) -> Result<TemplateEngine, TemplateError>
    where
        C: Into<TemplateOptions>,
    {
        let opt: TemplateOptions = opt.into();

        #[cfg(feature = "handlebars")]
        return HandlebarsTemplateEngine::register(opt);

        #[cfg(feature = "tera")]
        return TeraTemplateEngine::register(opt);

        Err(TemplateError::Registration(format!(
            "The template engine '{}' has no supporting driver.",
            opt.directory
        )))
    }
}

impl From<&str> for TemplateOptions {
    fn from(string: &str) -> TemplateOptions {
        TemplateOptions::from_str(string)
    }
}

impl From<&String> for TemplateOptions {
    fn from(string: &String) -> TemplateOptions {
        TemplateOptions::from_str(string.as_str())
    }
}

impl From<String> for TemplateOptions {
    fn from(string: String) -> TemplateOptions {
        TemplateOptions::new(string)
    }
}

impl TemplateOptions {
    /// Create new [ConnectOptions] for a [Database] by passing in a URI string
    pub fn new(directory: String) -> Self {
        Self { directory }
    }

    fn from_str(url: &str) -> Self {
        Self::new(url.to_owned())
    }

    /// Get the database URL of the pool
    pub fn get_directory(&self) -> &str {
        &self.directory
    }
}
