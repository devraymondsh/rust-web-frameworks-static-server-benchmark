use axum::{extract::Path, routing, Json, Router};
use logic;
use std::net::SocketAddr;

async fn index(Path(fibo_destination): Path<String>) -> Json<logic::FiboResults> {
    Json(logic::run_fibo(fibo_destination).await)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/:fibo_destination", routing::get(index));

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 9852)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
