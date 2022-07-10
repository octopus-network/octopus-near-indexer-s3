use crate::cache::raw::IndexerRawTable;
use crate::{DB_POOL, PROJECT_CONFIG};
use near_lake_framework::{LakeConfig, LakeConfigBuilder};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_subscriber::EnvFilter;
// use near_lake_framework::LakeConfigBuilder;

pub struct Env {
    pub(crate) start_block_height_from_cache: bool,
    pub(crate) start_block_height: i64,
    pub(crate) end_block_height: i64,
    pub(crate) push_engine: bool,
    pub(crate) push_engine_url: String,
    pub(crate) enable_http_server: bool,
    pub(crate) http_server_listen: String,
    pub(crate) database_url: String,
}

pub fn init_tracing() {
    let env_filter =
        EnvFilter::new("near_lake_framework=info,octopus-near-indexer-s3=info,http=info");

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .init();
}

pub async fn init_lake_config() -> LakeConfig {
    let mut current_height = match IndexerRawTable::select_current_height().await {
        Ok(cache) => cache.height,
        Err(_) => 0,
    };

    if !PROJECT_CONFIG.start_block_height_from_cache {
        current_height = PROJECT_CONFIG.start_block_height;
    }

    LakeConfigBuilder::default()
        .mainnet()
        .start_block_height(current_height as u64)
        .build()
        .expect("failed to start block height")

    // LakeConfig {
    //     s3_bucket_name: PROJECT_CONFIG.s3_bucket_name.clone(),
    //     s3_region_name: PROJECT_CONFIG.s3_region_name.clone(),
    //     start_block_height: current_height as u64,
    //     s3_config: None,
    // }
}

pub fn init_env_config() -> Env {
    Env {
        // s3_bucket_name: env::var("S3_BUCKET_NAME").unwrap(),
        // s3_region_name: env::var("S3_REGION_NAME").unwrap(),
        start_block_height_from_cache: env::var("START_BLOCK_HEIGHT_FROM_CACHE")
            .unwrap()
            .parse::<bool>()
            .unwrap(),
        start_block_height: env::var("START_BLOCK_HEIGHT")
            .unwrap()
            .parse::<i64>()
            .unwrap(),
        end_block_height: env::var("END_BLOCK_HEIGHT")
            .unwrap()
            .parse::<i64>()
            .unwrap(),
        push_engine: env::var("ENABLE_PUSH_SERVER")
            .unwrap()
            .parse::<bool>()
            .unwrap(),
        push_engine_url: env::var("PUSH_URL").unwrap(),
        enable_http_server: env::var("ENABLE_HTTP_SERVER")
            .unwrap()
            .parse::<bool>()
            .unwrap(),
        http_server_listen: env::var("HTTP_SERVER_LISTEN").unwrap(),
        database_url: env::var("DATABASE_URL").unwrap(),
    }
}

pub async fn init_db_pool() {
    let pool = PgPoolOptions::new()
        .connect(&PROJECT_CONFIG.database_url)
        .await
        .expect("Connect db fail");
    DB_POOL.set(pool).expect("Set db pool fail");
}
