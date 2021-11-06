use crate::templates::TemplateEngine;
use crate::templates::TemplateError;
use crate::templates::TemplateOptions;
use axum::response::Html;
use hyper::StatusCode;
use serde_json::Value;

#[cfg(feature = "handlebars")]
pub(crate) mod handlebars;
#[cfg(feature = "tera")]
pub(crate) mod tera;

#[cfg(feature = "handlebars")]
pub use handlebars::*;
#[cfg(feature = "tera")]
pub use tera::*;
