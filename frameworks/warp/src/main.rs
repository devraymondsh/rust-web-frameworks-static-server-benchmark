use warp::{Filter, http::Response};
use std::{path::Path, ffi::OsStr, convert::Infallible};

use include_dir::{include_dir, Dir};
static PROJECT_DIR: Dir = include_dir!("../../static");

pub async fn index(filename: String) -> Result<Response<String>, Infallible> {
    match PROJECT_DIR.get_file(&filename) {
        Some(contents) => {
            let content_type = match Path::new(&filename).extension().and_then(OsStr::to_str) {
                Some(ext) => mime_guess::from_ext(ext).first_or_text_plain().to_string(),
                None => String::from("text/plain"),
            };

            match contents.contents_utf8() {
                Some(contents) => Ok(Response::builder()
                    .header("Content-Type", content_type)
                    .body(String::from(contents)).unwrap()),
                None => Ok(Response::builder()
                    .status(404)
                    .header("Content-Type", "text/html")
                    .body(String::from("Failed to get the contents as UTF-8!")).unwrap()),
            }
        },
        None => Ok(Response::builder()
            .status(404)
            .header("Content-Type", "text/html")
            .body(String::from("File not found!")).unwrap())
    }
}

#[tokio::main]
async fn main() {
    let routes = warp::path!(String).and_then(index);

    warp::serve(routes).run(([127, 0, 0, 1], 9852)).await;
}
