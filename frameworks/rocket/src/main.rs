use rocket::{config, Config, http::{Status, ContentType}};
use std::{path::Path, ffi::OsStr, net::Ipv4Addr};

#[macro_use]
extern crate rocket;

use include_dir::{include_dir, Dir};
static PROJECT_DIR: Dir = include_dir!("../../static");

#[get("/<filename>")]
async fn index(filename: String) -> (Status, (ContentType, String)) {
    
    let mut status = Status::Ok;
    let mut mime = ContentType::HTML;
    let body = match PROJECT_DIR.get_file(&filename) {
        Some(contents) => {
            let content_type = match Path::new(&filename).extension().and_then(OsStr::to_str) {
                Some(ext) => ext,
                None => "text/plain",
            };

            match contents.contents_utf8() {
                Some(contents) => {
                    mime = ContentType::from_extension(content_type).unwrap_or(ContentType::Text);

                    String::from(contents)
                },
                None => {
                    status = Status::NotFound;

                    String::from("Failed to get the contents as UTF-8!")
                },
            }
        },
        None => {
            status = Status::NotFound;

            String::from("File not found!")
        }
    };

    (status, (mime, body))
}

#[launch]
fn rocket() -> _ {
    let cfg = Config {
        port: 9852,
        log_level: config::LogLevel::Off,
        address: Ipv4Addr::new(127, 0, 0, 1).into(),
        ..Config::debug_default()
    };

    rocket::custom(cfg).mount("/", routes![index])
}
