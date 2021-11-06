use crate::templates::TemplateEngine;
use axum::AddExtensionLayer;
use axum::Router;
use sea_orm::DatabaseConnection;
use tower::ServiceBuilder;

#[derive(Clone)]
pub struct Application {
    // router for your application
    // pub router: Router,

    // template engine for your application
    pub templates: TemplateEngine,

    // database connections for your application
    pub database: DatabaseConnection,
}

impl Application {
    // pub fn configure(&self) -> axum::Router {
    //     self.router.layer(
    //         ServiceBuilder::new()
    //             .layer(AddExtensionLayer::new(self.templates))
    //             .layer(AddExtensionLayer::new(self.database)),
    //     )
    // }
}

// impl Default for Application {
//     fn default() -> Self {
//         Self {}
//     }
// }
