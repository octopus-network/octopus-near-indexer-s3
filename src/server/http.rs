use salvo::extra::logging::LogHandler;
use salvo::prelude::*;

#[fn_handler]
async fn not_found(res: &mut Response) {
    res.render_json_text("{}");
}

pub async fn services() {
    let router = Router::with_hoop(LogHandler).push(Router::new().path("<**>").handle(not_found));

    let service = Service::new(router);

    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(service)
        .await;
}
