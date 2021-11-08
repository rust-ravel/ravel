use actix_web::HttpResponse;
use serde_json::Value;

pub struct Response {
    inner: HttpResponse,
}

impl Into<HttpResponse> for Response {
    fn into(self) -> HttpResponse {
        self.inner
    }
}

impl Response {
    pub fn json(data: Value) -> Self {
        Response {
            inner: HttpResponse::Ok().json(data),
        }
    }

    pub fn render(view: &str, json: Value) -> Self {
        let compiled_template = "<h1>TODO<h1>";

        Response {
            inner: HttpResponse::Ok()
                .content_type("text/html")
                .body(compiled_template),
        }
    }

    pub fn html(html: &'static str) -> Self {
        Response {
            inner: HttpResponse::Ok().content_type("text/html").body(html),
        }
    }

    pub fn text(text: &'static str) -> Self {
        Response {
            inner: HttpResponse::Ok().body(text),
        }
    }

    pub fn no_content() -> Self {
        Response {
            inner: HttpResponse::Ok().finish(),
        }
    }
}
