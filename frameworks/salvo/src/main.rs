use logic;
use salvo::{
    async_trait,
    prelude::{fn_handler, Router, Server, TcpListener},
    writer::Json,
    Request, Response,
};

#[fn_handler]
async fn index(req: &mut Request, res: &mut Response) {
    let fibo_destination = req.param::<String>("fibo_destination").unwrap();

    res.render(Json(logic::run_fibo(fibo_destination).await));
}

#[tokio::main]
async fn main() {
    let router = Router::with_path("<fibo_destination>").get(index);

    Server::new(TcpListener::bind("127.0.0.1:9852"))
        .serve(router)
        .await;
}
