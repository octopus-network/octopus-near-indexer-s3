use anyhow::Result;
use dotenv::dotenv;
use futures::join;
use octopus_near_indexer_s3::config::{init_lake_config, init_tracing};
use octopus_near_indexer_s3::indexer::stream::indexer_stream;
use octopus_near_indexer_s3::server::http::services;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init_tracing();

    let services = services();
    let indexer = indexer_stream(init_lake_config()?);

    join!(services, indexer);
    Ok(())
}
