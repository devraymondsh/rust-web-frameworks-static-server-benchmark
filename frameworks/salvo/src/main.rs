use salvo::{
    prelude::{handler, Router, Server, TcpListener, StatusCode},
    writer::Text,
    Request, Response,
};
use std::{path::Path, ffi::OsStr};

use include_dir::{include_dir, Dir};
static PROJECT_DIR: Dir = include_dir!("../../static");

#[handler]
async fn index(req: &mut Request, res: &mut Response) {
    let path = req.param::<String>("filename").unwrap();

    match PROJECT_DIR.get_file(&path) {
        Some(contents) => {
            let content_type = match Path::new(&path).extension().and_then(OsStr::to_str) {
                Some(ext) => mime_guess::from_ext(ext).first_or_text_plain(),
                None => mime::TEXT_PLAIN,
            };

            match contents.contents_utf8() {
                Some(contents) => {
                    let body = if content_type == mime::TEXT_PLAIN {
                        Text::Plain(contents.to_string());
                    } else if content_type == mime::APPLICATION_JAVASCRIPT {
                        Text::Json(contents.to_string());
                    } else if content_type == mime::TEXT_XML {
                        Text::Xml(contents.to_string());
                    }else if content_type == mime::TEXT_HTML {
                        Text::Html(contents.to_string());
                    } else if content_type == mime::TEXT_JAVASCRIPT {
                        Text::Js(contents.to_string());
                    } else if content_type == mime::TEXT_CSS {
                        Text::Css(contents.to_string());
                    } else if content_type == mime::TEXT_CSV {
                        Text::Csv(contents.to_string());
                    } else {
                        Text::Plain(contents.to_string());
                    };

                    res.render(body);
                },
                None => {
                    res.set_status_code(StatusCode::NOT_FOUND);
                    res.render(Text::Html("Failed to get the contents as UTF-8!"));
                },
            }
        },
        None => {
            res.set_status_code(StatusCode::NOT_FOUND);
            res.render(Text::Html("File not found!"));
        }
    };
}

#[tokio::main]
async fn main() {
    let router = Router::with_path("<filename>").get(index);

    Server::new(TcpListener::bind("127.0.0.1:9852"))
        .serve(router)
        .await;
}
