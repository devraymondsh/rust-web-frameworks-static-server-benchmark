use hyper::{
    header::{HeaderName, HeaderValue},
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{convert::Infallible, str::FromStr, path::Path, ffi::OsStr};

use include_dir::{include_dir, Dir};
static PROJECT_DIR: Dir = include_dir!("../../static");

async fn index(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri_mut().path().replace("/", "");

    let mut mime = String::from("text/html");
    let body = match PROJECT_DIR.get_file(&path) {
        Some(contents) => {
            let content_type = match Path::new(&path).extension().and_then(OsStr::to_str) {
                Some(ext) => mime_guess::from_ext(ext).first_or_text_plain().to_string(),
                None => String::from("text/plain"),
            };

            match contents.contents_utf8() {
                Some(contents) => {
                    mime = content_type;

                    String::from(contents)
                },
                None => String::from("Failed to get the contents as UTF-8!"),
            }
        },
        None => {
            String::from("File not found!")
        }
    };
    
    let mut res = Response::new(Body::from(body));
    res.headers_mut().append(
        HeaderName::from_str("Content-Type").unwrap(),
        HeaderValue::from_str(mime.as_str()).unwrap(),
    );

    Ok(res)
}

#[tokio::main]
pub async fn main() {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(index)) });

    Server::bind(&([127, 0, 0, 1], 9852).into())
        .serve(make_svc)
        .await
        .unwrap()
}
