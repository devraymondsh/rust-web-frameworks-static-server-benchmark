use futures_util::future::FutureExt;
use gotham::{
    handler::HandlerFuture,
    helpers::http::response,
    hyper::StatusCode,
    prelude::{DefineSingleRoute, DrawRoutes, FromState},
    router::{builder, response::StaticResponseExtender},
    state::{State, StateData},
};
use serde::Deserialize;
use std::{path::Path, ffi::OsStr, pin::Pin};

use include_dir::{include_dir, Dir};
static PROJECT_DIR: Dir = include_dir!("../../static");

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct IndexPathParams {
    filename: String,
}

fn index(mut state: State) -> Pin<Box<HandlerFuture>> {
    let IndexPathParams { filename } = IndexPathParams::take_from(&mut state);

    async move {
        let mut mime = mime::TEXT_HTML;
        let mut status = StatusCode::OK;
        let body = match PROJECT_DIR.get_file(&filename) {
            Some(contents) => {
                let content_type = match Path::new(&filename).extension().and_then(OsStr::to_str) {
                    Some(ext) => mime_guess::from_ext(ext).first_or_text_plain(),
                    _ => mime::TEXT_PLAIN,
                };
    
                match contents.contents_utf8() {
                    Some(contents) => {
                        mime = content_type;

                        String::from(contents)
                    },
                    None => {
                        status = StatusCode::NOT_FOUND;

                        String::from("Failed to get the contents as UTF-8!")
                    },
                }
            },
            None => {
                status = StatusCode::NOT_FOUND;

                String::from("File not found!")
            }
        };

        let response =
            response::create_response(&state, status, mime, body);

        Ok((state, response))
    }.boxed()
}

fn main() {
    gotham::start(
        "127.0.0.1:9852",
        builder::build_simple_router(|route| {
            route
                .get("/:filename")
                .with_path_extractor::<IndexPathParams>()
                .to(index);
        }),
    )
    .unwrap()
}
