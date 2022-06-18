use futures_util::future::{self, FutureExt};
use gotham::{
    handler::HandlerFuture,
    helpers::http::response,
    hyper::StatusCode,
    prelude::{DefineSingleRoute, DrawRoutes, FromState},
    router::{builder, response::StaticResponseExtender},
    state::{State, StateData},
};
use logic;
use serde::Deserialize;
use std::pin::Pin;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct IndexPathParams {
    fibo_destination: String,
}

fn index(mut state: State) -> Pin<Box<HandlerFuture>> {
    let IndexPathParams { fibo_destination } = IndexPathParams::take_from(&mut state);

    logic::run_fibo(fibo_destination)
        .then(|fibo_results| {
            let body = serde_json::to_string(&fibo_results).unwrap();

            let response =
                response::create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body);

            future::ok((state, response))
        })
        .boxed()
}

fn main() {
    gotham::start(
        "127.0.0.1:9852",
        builder::build_simple_router(|route| {
            route
                .get("/:fibo_destination")
                .with_path_extractor::<IndexPathParams>()
                .to(index);
        }),
    )
    .unwrap()
}
