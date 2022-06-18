use hyper::{
    header::{HeaderName, HeaderValue},
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use logic;
use std::{convert::Infallible, str::FromStr};

async fn index(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let fibo_destination = req.uri_mut().path().replace("/", "");

    let fibo_results = logic::run_fibo(fibo_destination).await;

    let body = serde_json::to_string(&fibo_results).unwrap();

    let mut res = Response::new(Body::from(body));
    res.headers_mut().append(
        HeaderName::from_str("Content-Type").unwrap(),
        HeaderValue::from_str("application/json").unwrap(),
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
