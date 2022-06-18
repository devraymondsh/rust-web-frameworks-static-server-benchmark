use actix_web::{get, App, web::{Path, Json}, HttpServer, Responder};
use logic;

#[get("/{fibo_destination}")]
async fn index(fibo_destination: Path<String>) -> impl Responder {
    Json(logic::run_fibo(fibo_destination.to_string()).await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:9852")?
        .run()
        .await
}
