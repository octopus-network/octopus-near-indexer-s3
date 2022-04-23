use crate::cache::raw::IndexerRawTable;
use crate::server::res::Res;
use crate::PROJECT_CONFIG;
use salvo::extra::logging::LogHandler;
use salvo::prelude::*;
use serde_json::json;

#[fn_handler]
async fn current_height(res: &mut Response) {
    let select = IndexerRawTable::select_current_height().await;
    match select {
        Ok(data) => Res::default()
            .data(json!({ "height": data.height, "block": data }))
            .ok(res),
        Err(_) => Res::default()
            .data(json!({"height": 0, "block": {} }))
            .ok(res),
    }
}

pub async fn services() {
    let router =
        Router::with_hoop(LogHandler).push(Router::new().path("/cache").get(current_height));
    let service = Service::new(router);
    Server::new(TcpListener::bind(&PROJECT_CONFIG.http_server_listen))
        .serve(service)
        .await;
}
