use logic;
use warp::{reply::json, Filter, Rejection, Reply};

pub async fn index(fibo_destination: String) -> Result<impl Reply, Rejection> {
    let fibo_results = logic::run_fibo(fibo_destination).await;

    Ok(json(&fibo_results))
}

#[tokio::main]
async fn main() {
    let routes = warp::path!(String).and_then(index);

    warp::serve(routes).run(([127, 0, 0, 1], 9852)).await;
}
