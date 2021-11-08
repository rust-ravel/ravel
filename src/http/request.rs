pub struct Request {}

impl From<actix_web::HttpRequest> for Request {
    fn from(_: actix_web::HttpRequest) -> Self {
        Self {}
    }
}
