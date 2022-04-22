use crate::indexer::engine::push_block_to_engine;
use crate::INDEXER;
use futures::StreamExt;
use near_lake_framework::LakeConfig;

pub async fn indexer_stream(config: LakeConfig) {
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
    // TODO: catch error and retry
    // push_block_to_engine(streamer_message).await.unwrap();
}
