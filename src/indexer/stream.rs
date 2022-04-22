use std::env;
use std::str::ParseBoolError;
use crate::cache::raw::{IndexerRaw, RawTableStruct};
use crate::indexer::engine::push_block_to_engine;
use crate::INDEXER;
use futures::StreamExt;
use near_lake_framework::LakeConfig;
use serde_json::json;

// pub async fn indexer_start(config: LakeConfig) {
//     let cache_current_height = match IndexerRaw::select_current_height().await {
//         Ok(cache) => cache.height,
//         Err(_) => 0
//     };
// }

pub async fn indexer_stream_from_s3(config: LakeConfig) {
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
    let raw = RawTableStruct {
        prev_hash: streamer_message.block.header.prev_hash.to_string(),
        height: streamer_message.block.header.height,
        hash: streamer_message.block.header.hash.to_string(),
        raw: json!(streamer_message),
    };

    IndexerRaw::insert(raw).await.unwrap();

    if env::var("ENGINE_HTTP")?.parse::<bool>()? {
        push_block_to_engine(streamer_message).await.unwrap();
    }
}
