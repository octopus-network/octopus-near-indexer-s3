use anyhow::Result;
use dotenv::dotenv;
use futures::StreamExt;
use octopus_near_indexer_s3::config::{init_lake_config, init_tracing};
use octopus_near_indexer_s3::engine::push_block_to_engine;
use octopus_near_indexer_s3::INDEXER;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init_tracing();

    let stream = near_lake_framework::streamer(init_lake_config()?);

    let mut handlers = tokio_stream::wrappers::ReceiverStream::new(stream)
        .map(handle_streamer_message)
        .buffer_unordered(1usize);

    while let Some(_handle_message) = handlers.next().await {}

    Ok(())
}

async fn handle_streamer_message(
    streamer_message: near_lake_framework::near_indexer_primitives::StreamerMessage,
) {
    tracing::info!(
        target: INDEXER,
        "{} / shards {}",
        streamer_message.block.header.height,
        streamer_message.shards.len()
    );
    // TODO: catch error and retry
    push_block_to_engine(streamer_message).await.unwrap();
}
