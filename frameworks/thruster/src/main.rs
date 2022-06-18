use logic;
use thruster::{
    m, middleware_fn, App, BasicContext as Ctx, MiddlewareNext, MiddlewareResult, Request, Server,
    ThrusterServer,
};

#[middleware_fn]
async fn index(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    // The context.params is always None sadly.
    let fibo_destination = context.request.path().replace("/", "");

    let fibo_results = logic::run_fibo(fibo_destination).await;

    context
        .headers
        .insert("Content-Type".to_owned(), "application/json".to_owned());

    context.body(serde_json::to_string(&fibo_results).unwrap().as_str());

    Ok(context)
}

fn main() {
    let app = App::<Request, Ctx, ()>::new_basic().get("/:fibo_destination", m![index]);

    let server = Server::new(app);

    server.start("127.0.0.1", 9852);
}
