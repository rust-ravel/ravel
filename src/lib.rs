use std::convert::Infallible;
pub use application::Application;
use axum::body::BoxBody;
use axum::body::Bytes;
use axum::handler::Handler;
use axum::BoxError;

use axum::body::Body;
use axum::error_handling::HandleErrorLayer;
use axum::http::{Request, Response};
use axum::response::IntoResponse;
use axum::AddExtensionLayer;
use axum::Router;
use log::{debug, error, info, log_enabled, Level};
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use std::env;
use templates::Template;
use tower::filter::AsyncFilterLayer;
use tower::util::AndThenLayer;
use tower::ServiceBuilder;

pub mod application;
pub mod macros;
pub mod templates;

// reexports
pub use axum;
pub use hyper;
pub use log::*;
pub use sea_orm;
pub use tokio;
pub use tower;
use crate::axum::http::{Extensions, StatusCode};
use crate::errors::HttpError;

pub async fn boot(router: Router) {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let app_url = env::var("APP_URL").expect("APP_URL is not set in .env file");

    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("Failed to connect to database.");

    let view_path = format!("{}/src/views", base_path!());
    let template =
        Template::register(view_path.as_str()).expect("Failed to register template engine.");

    let application = Application {
        database: db,
        templates: template,
    };

    let app = router
        // .fallback(handler_404.into_service())
        .layer(
            ServiceBuilder::new()
                // .layer(HandleErrorLayer::new(|error| {
                //     (
                //         StatusCode::INTERNAL_SERVER_ERROR,
                //         format!("Unhandled internal error: {}", error),
                //     )
                // }))
                // .layer(AndThenLayer::new(map_response))
                // .layer(AsyncFilterLayer::new(map_request))
                // This middleware most be added above any fallible
                // ones if you're using `ServiceBuilder`, due to how ordering works
                // .layer(HandleErrorLayer::new(HandleError { inner: todo!(), f: todo!(), _marker: todo!() }))
                .layer(AddExtensionLayer::new(application)),
        );

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {}", addr);
    axum::Server::bind(&app_url.parse().unwrap())
        .serve(app.into_make_service())
        .await;
}

// impl IntoResponse for StatusCode {
//     type Body = ();
//     type BodyError = ();
//
//     fn into_response(self) -> Response<Self::Body> {
//         (self::)
//     }
// }
// async fn handler_404() -> impl IntoResponse {
//     (StatusCode::NOT_FOUND, "nothing to see here?!")
// }
//
// async fn map_request(req: Request<Body>) -> Result<Request<Body>, BoxError> {
//     let (parts, body) = req.into_parts();
//     let bytes = buffer_and_print("request", body).await?;
//     let req = Request::from_parts(parts, Body::from(bytes));
//     Ok(req)
// }
//
// async fn map_response(res: Response<BoxBody>) -> Result<Response<Body>, BoxError> {
//     debug!("RESPONDING WITH: {:?}", res);
//     let (parts, body) = res.into_parts();
//     if !parts.status.is_success() {
//         return Ok(Response::new(Body::from(format!("Error code: {}", parts.status.as_u16()))));
//     }
//
//     let bytes = buffer_and_print("response", body).await?;
//     let res = Response::from_parts(parts, Body::from(bytes));
//     Ok(res)
// }
//
// async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, BoxError>
// where
//     B: axum::body::HttpBody<Data = Bytes>,
//     B::Error: Into<BoxError>,
// {
//     let bytes = hyper::body::to_bytes(body).await.map_err(Into::into)?;
//     if let Ok(body) = std::str::from_utf8(&bytes) {
//         debug!("{} body = {:?}", direction, body);
//     }
//     Ok(bytes)
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
