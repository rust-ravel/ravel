pub use application::Application;

use std::env;

pub mod application;
pub mod http;
pub mod macros;
pub mod templates;

// reexports
// pub use actix_web;
// pub use actix_web::main;
pub use log::*;
pub use sea_orm;

pub async fn boot() -> Application {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv::dotenv().ok();

    Application::build().await
}

// #[cfg(test)]
// mod tests {
//     use crate::http::Request;

//     use super::*;
//     use actix_web::{test, web, App};

//     #[actix_rt::test]
//     async fn test_index_get() {
//         // let mut app = test::init_service(
//         //     App::new()
//         //         .data(AppState { count: 4 })
//         //         .route("/", web::get().to(index)),
//         // )
//         // .await;
//         // let req = test::TestRequest::get().uri("/").to_request();
//         // let resp: AppState = test::read_response_json(&mut app, req).await;

//         let application = Application::build();

//         let request = Request {};

//         let response = application.handle();

//         assert_eq!(resp.count, 4);
//     }
// }
