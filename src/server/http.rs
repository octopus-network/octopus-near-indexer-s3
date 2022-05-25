use crate::cache::raw::IndexerRawTable;
use crate::server::res::Res;
use crate::PROJECT_CONFIG;
use salvo::extra::logging::LogHandler;
use salvo::prelude::*;
use serde_json::json;

#[fn_handler]
async fn current_height(res: &mut Response) {
    let current = IndexerRawTable::select_current_height().await;
    match current {
        Ok(data) => Res::default()
            .data(json!({ "height": data.height, "block": data }))
            .ok(res),
        Err(_) => Res::default()
            .data(json!({"height": 0, "block": {} }))
            .ok(res),
    }
}

#[fn_handler]
async fn cache(req: &mut Request, res: &mut Response) {
    let id = req.get_param("id").unwrap();
    let local_cache = IndexerRawTable::select_from_height(id).await;
    match local_cache {
        Ok(data) => Res::default()
            .data(json!({ "height": data.height, "block": data }))
            .ok(res),
        Err(_) => Res::default()
            .data(json!({"height": 0, "block": {} }))
            .ok(res),
    }
}

pub async fn services() {
    if !PROJECT_CONFIG.enable_http_server {
        return;
    }
    let router = Router::with_hoop(LogHandler)
        .push(Router::new().path("/cache").get(current_height))
        .push(Router::new().path("/cache/<id>").get(cache));
    let service = Service::new(router);
    Server::new(TcpListener::bind(&PROJECT_CONFIG.http_server_listen))
        .serve(service)
        .await;
}
