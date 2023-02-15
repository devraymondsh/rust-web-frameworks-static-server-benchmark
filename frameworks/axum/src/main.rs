use axum::{
    http::{StatusCode, header::{HeaderName, HeaderValue}},
    Router,
    routing::get,
    extract::Path as AxumPath,
    response::{IntoResponse, IntoResponseParts, ResponseParts, Response}
};
use std::{path::Path, ffi::OsStr, net::SocketAddr};

use include_dir::{include_dir, Dir};
static PROJECT_DIR: Dir = include_dir!("../../static");

// Hypothetical helper type for setting a single header
struct SetHeader(String, String);
impl IntoResponseParts for SetHeader {
    type Error = (StatusCode, String);

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        match (self.0.parse::<HeaderName>(), self.1.parse::<HeaderValue>()) {
            (Ok(name), Ok(value)) => {
                res.headers_mut().insert(name, value);
            },
            (Err(_), _) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header name {}", self.0),
                ));
            },
            (_, Err(_)) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header value {}", self.1),
                ));
            },
        }

        Ok(res)
    }
}
// Its also recommended to implement `IntoResponse` so `SetHeader` can be used on its own as
// the response
impl IntoResponse for SetHeader {
    fn into_response(self) -> Response {
        // This gives an empty response with the header
        (self, ()).into_response()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:filename", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9852));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(AxumPath(path): AxumPath<String>) -> (StatusCode, SetHeader, String) {
    match PROJECT_DIR.get_file(&path) {
        Some(contents) => {
            let content_type = match Path::new(&path).extension().and_then(OsStr::to_str) {
                Some(ext) => mime_guess::from_ext(ext).first_or_text_plain().to_string(),
                None => String::from("text/plain"),
            };

            match contents.contents_utf8() {
                Some(contents) => (
                    StatusCode::OK,
                    SetHeader(String::from("Content-Type"), String::from(content_type)),
                    String::from(contents),
                ),
                None => (
                    StatusCode::NOT_FOUND,
                    SetHeader(String::from("Content-Type"), String::from("text/html")),
                    String::from("Failed to get the contents as UTF-8!"),
                ),
            }
        },
        None => (
            StatusCode::NOT_FOUND,
            SetHeader(String::from("Content-Type"), String::from("text/html")),
            String::from("File not found!"),
        )
    }
}