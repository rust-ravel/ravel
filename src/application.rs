use std::{env, io::Error};

use crate::{
    base_path,
    http::*,
    templates::{Template, TemplateEngine},
};
use actix_files::Files;
use actix_web::{middleware, web::Data, App, HttpServer};
use log::debug;
use sea_orm::{Database, DatabaseConnection};

#[derive(Debug, Clone)]
pub struct Application {
    // template engine for your application
    pub(in crate) templates: TemplateEngine,

    // database connections for your application
    pub database: DatabaseConnection,

    router: Router,

    url: String,
    // server: HttpServer,
}

impl Application {
    pub fn nest(&mut self, uri: Uri, router: Router) -> &mut Self {
        self.router.nest(uri, router);
        self
    }

    pub fn get(&mut self, uri: Uri, action: Action) -> &mut Self {
        self.router.get(uri, action);
        self
    }

    pub fn post(&mut self, uri: Uri, action: Action) -> &mut Self {
        self.router.post(uri, action);
        self
    }

    pub fn put(&mut self, uri: Uri, action: Action) -> &mut Self {
        self.router.put(uri, action);
        self
    }

    pub fn delete(&mut self, uri: Uri, action: Action) -> &mut Self {
        self.router.delete(uri, action);
        self
    }

    pub async fn build() -> Self {
        let app_url = env::var("APP_URL").expect("APP_URL is not set in .env file");

        Self {
            templates: Application::build_template_engine(),
            database: Application::build_database().await,
            url: app_url,
            router: Router::new(), // server: HttpServer::new(factory),
        }
    }

    pub async fn start(self) -> Result<(), Error> {
        let app_url = self.url.clone();

        debug!("Routers: {:?}", self.router);

        let server = HttpServer::new(move || {
            App::new()
                .app_data(Data::new(self.clone()))
                // TODO: bake these into the public API: .service(Files::new("/public", "./public"))
                // TODO: bake these into the public API: .wrap(middleware::Logger::default()) // enable logger
                .configure(|cnf| self.router.clone().config_services(cnf))
        });

        server.bind(app_url)?.run().await
    }

    /// build the default database connection based on environment variables
    pub async fn build_database() -> DatabaseConnection {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

        Database::connect(db_url)
            .await
            .expect("Failed to connect to database.")
    }

    /// build the default database connection based on environment variables
    pub fn build_template_engine() -> TemplateEngine {
        let view_path = format!("{}/resources/views", base_path!());
        Template::register(view_path.as_str())
    }
}
