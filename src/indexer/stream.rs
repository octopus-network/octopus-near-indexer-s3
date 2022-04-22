use crate::cache::raw::{IndexerRaw, RawTableStruct};
use crate::config::init_lake_config;
use crate::pusher::http::push_block_to_engine;
use crate::INDEXER;
use futures::StreamExt;
use serde_json::json;

pub async fn indexer_stream_from_s3() {
    let config = init_lake_config().await.unwrap();

    let stream = near_lake_framework::streamer(config);

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

    let json = json!(streamer_message);

    let raw = RawTableStruct {
        prev_hash: streamer_message.block.header.prev_hash.to_string(),
        height: streamer_message.block.header.height as i64,
        hash: streamer_message.block.header.hash.to_string(),
        raw: json.clone(),
    };

    IndexerRaw::insert(raw).await.unwrap();

    push_block_to_engine(&json).await.unwrap();
}
