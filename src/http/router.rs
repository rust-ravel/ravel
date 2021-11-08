use std::fmt::Debug;

use actix_web::web::{self, Data};
use async_trait::async_trait;
use sea_orm::ColumnTrait;

use crate::Application;

use super::{Request, Response};

// temp for now so I can build out the proper DX.
pub type Action = fn(Request) -> Response; //Box<dyn Fn(Request) -> Response>;

// #[async_trait]
// pub trait Action: Debug {
//     /// Call the handler with the given request.
//     async fn call(self, req: Request) -> Response;
// }

pub type Uri = &'static str;

#[derive(Debug, Clone)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Router::default()
    }

    pub fn get(&mut self, uri: Uri, action: Action) -> &mut Self {
        self.add_route(Route::new(Method::Get, uri, action));
        self
    }

    pub fn post(&mut self, uri: Uri, action: Action) -> &mut Self {
        self.add_route(Route::new(Method::Post, uri, action));
        self
    }

    pub fn put(&mut self, uri: Uri, action: Action) -> &mut Self {
        self.add_route(Route::new(Method::Put, uri, action));
        self
    }

    pub fn delete(&mut self, uri: Uri, action: Action) -> &mut Self {
        self.add_route(Route::new(Method::Delete, uri, action));
        self
    }

    pub fn nest(&mut self, uri: Uri, router: Router) -> &mut Router {
        for mut route in router.routes {
            // If they will not cause a double // in between paths.
            if !(route.uri.starts_with("/") && uri.ends_with("/")) {
                let mut prefix = uri.to_string();
                prefix.push_str(route.uri.as_str());
                route.uri = prefix;
            }
            self.add_route(route);
        }

        self
    }

    /// Add a new route instance to this router
    pub(in crate) fn add_route(&mut self, route: Route) -> &mut Router {
        self.routes.push(route);

        self
    }

    pub(in crate) fn config_services(self, config: &mut web::ServiceConfig) {
        for route in self.routes {
            config.route(route.uri.clone().as_str(), route.to_actix());
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self { routes: vec![] }
    }
}

#[derive(Clone, Debug)]
pub struct Route {
    method: Method,
    uri: String,
    action: Action,
}

impl Route {
    pub fn get(uri: Uri, action: Action) -> Route {
        Self::new(Method::Get, uri, action)
    }

    pub fn post(uri: Uri, action: Action) -> Route {
        Self::new(Method::Post, uri, action)
    }

    pub fn put(uri: Uri, action: Action) -> Route {
        Self::new(Method::Put, uri, action)
    }

    pub fn delete(uri: Uri, action: Action) -> Route {
        Self::new(Method::Delete, uri, action)
    }

    pub fn new(method: Method, uri: Uri, action: Action) -> Self {
        Self {
            method,
            uri: String::from(uri),
            action,
        }
    }

    pub(crate) fn to_actix(self) -> actix_web::Route {
        let actix_handler = move |actix_request: actix_web::HttpRequest,
                                  app_state: Data<Application>|
              -> actix_web::HttpResponse {
            let request = Request::from(actix_request);
            let response = (self.action)(request);
            response.into()
        };

        actix_web::Route::new()
            .method(actix_web::http::Method::GET)
            .to(actix_handler)
    }
}

#[derive(Debug, Clone)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

#[cfg(test)]
mod tests {
    use crate::http::{router::Action, Request, Response, Route};

    use super::Router;

    #[actix_rt::test]
    async fn it_router_with_struct() {
        let mut router = Router::new();

        struct MyController {}

        impl MyController {
            pub fn index(request: Request) -> Response {
                Response::no_content()
            }
        }

        let route = Route::get("/test", MyController::index);
        router.add_route(route);

        assert_eq!(1, 1);
    }

    #[actix_rt::test]
    async fn it_router_with_closure() {
        let mut router = Router::new();

        // struct UserController
        let handler: Action = |request: Request| Response::no_content();

        let route = Route::get("/test", handler);
        router.add_route(route);

        assert_eq!(1, 1);
    }
}
