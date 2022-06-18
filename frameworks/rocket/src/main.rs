use rocket::{config, serde::json::Json, Config};
use std::net::Ipv4Addr;
use logic;

#[macro_use]
extern crate rocket;

#[get("/<fibo_destination>")]
async fn index(fibo_destination: String) -> Json<logic::FiboResults> {
    Json(logic::run_fibo(fibo_destination).await)
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
