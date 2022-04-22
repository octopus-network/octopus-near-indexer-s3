use salvo::extra::logging::LogHandler;
use salvo::prelude::*;
use std::env;

#[fn_handler]
async fn not_found(res: &mut Response) {
    res.render_json_text("{}");
}

pub async fn services() {
    let router = Router::with_hoop(LogHandler).push(Router::new().path("<**>").handle(not_found));
    let service = Service::new(router);
    Server::new(TcpListener::bind(
        &env::var("HTTP_LISTEN").expect("HTTP_LISTEN config fail"),
    ))
    .serve(service)
    .await;
}
