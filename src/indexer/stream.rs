use crate::cache::raw::{IndexerRawTable, IndexerRawTableStruct};
use crate::config::init_lake_config;
use crate::pusher::http::push_block_to_engine;
use crate::{INDEXER, PROJECT_CONFIG};
use futures::StreamExt;
use serde_json::json;
use std::process::exit;

pub async fn indexer_stream_from_s3() {
    let config = init_lake_config().await;

    let (_, stream) = near_lake_framework::streamer(config);

    let mut handlers = tokio_stream::wrappers::ReceiverStream::new(stream)
        .map(handle_streamer_message)
        .buffer_unordered(1usize);

    while let Some(_handle_message) = handlers.next().await {}
}

pub async fn handle_streamer_message(
    streamer_message: near_lake_framework::near_indexer_primitives::StreamerMessage,
) {
    tracing::info!(
        target: INDEXER,
        "{} / shards {}",
        streamer_message.block.header.height,
        streamer_message.shards.len()
    );

    if streamer_message.block.header.height as i64 > PROJECT_CONFIG.end_block_height {
        tracing::info!(
            target: INDEXER,
            "End sync for end height {} config",
            PROJECT_CONFIG.end_block_height
        );
        exit(0);
    }

    let json = json!(streamer_message);

    let raw = IndexerRawTableStruct {
        prev_hash: streamer_message.block.header.prev_hash.to_string(),
        height: streamer_message.block.header.height as i64,
        hash: streamer_message.block.header.hash.to_string(),
        raw: json.clone(),
    };

    IndexerRawTable::insert(raw).await.expect("Insert db fail");

    push_block_to_engine(&json)
        .await
        .expect("Push block http fail");
}
