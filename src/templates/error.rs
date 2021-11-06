use axum::response::IntoResponse;

/// An error from unsuccessful template operation.
#[derive(Debug, PartialEq)]
pub enum TemplateError {
    /// There was a problem registering the template engine.
    Registration(String),
    /// There was a problem registering a specific template.
    Render(String),
}

// impl IntoResponse for TemplateError {
//     type Body;

//     type BodyError;

//     fn into_response(self) -> hyper::Response<Self::Body> {
//         todo!()
//     }
// }
