use crate::DB_POOL;
use anyhow::Result;
use near_lake_framework::LakeConfig;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_subscriber::EnvFilter;

pub fn init_tracing() {
    let env_filter =
        EnvFilter::new("near_lake_framework=info,octopus-near-indexer-s3=info,http=info");

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .init();
}

pub fn init_lake_config() -> Result<LakeConfig> {
    let lake_config = LakeConfig {
        s3_endpoint: None,
        s3_bucket_name: env::var("S3_BUCKET_NAME")?,
        s3_region_name: env::var("S3_REGION_NAME")?,
        start_block_height: env::var("START_BLOCK_HEIGHT")?.parse::<u64>()?,
    };
    Ok(lake_config)
}

pub async fn init_db_pool() {
    let pool = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    DB_POOL.set(pool).unwrap();
}
