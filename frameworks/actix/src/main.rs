use actix_web::{get, HttpResponse, App, HttpServer, web};
use std::{path::Path, ffi::OsStr};

use include_dir::{include_dir, Dir};
static PROJECT_DIR: Dir = include_dir!("../../static");

#[get("/{filename}")]
async fn index(req: web::Path<String>) -> HttpResponse {
    let path = req.to_string();

    match PROJECT_DIR.get_file(&path) {
        Some(contents) => {
            let content_type = match Path::new(&path).extension().and_then(OsStr::to_str) {
                Some(ext) => mime_guess::from_ext(ext).first_or_text_plain().to_string(),
                None => String::from("text/plain"),
            };

            match contents.contents_utf8() {
                Some(contents) => {
                    return HttpResponse::Ok()
                    .insert_header(("Content-Type", content_type))
                    .body(String::from(contents))
                },
                None => return HttpResponse::NotFound()
                .insert_header(("Content-Type", "text/html"))
                .body("Failed to get the contents as UTF-8!"),
            }
        },
        None => {
            return HttpResponse::NotFound()
            .insert_header(("Content-Type", "text/html"))
            .body("File not found!")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 9852))?
        .run()
        .await
}
