use crate::cache::raw::IndexerRawTable;
use crate::{DB_POOL, PROJECT_CONFIG};
use anyhow::Result;
use near_lake_framework::LakeConfig;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_subscriber::EnvFilter;

pub struct Env {
    pub(crate) s3_endpoint: Option<String>,
    pub(crate) s3_bucket_name: String,
    pub(crate) s3_region_name: String,
    pub(crate) start_block_height_from_cache: bool,
    pub(crate) start_block_height: i64,
    pub(crate) push_engine: bool,
    pub(crate) push_engine_url: String,
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

pub async fn init_lake_config() -> Result<LakeConfig> {
    let mut current_height = match IndexerRawTable::select_current_height().await {
        Ok(cache) => cache.height,
        Err(_) => 0,
    };

    if !PROJECT_CONFIG.start_block_height_from_cache {
        current_height = PROJECT_CONFIG.start_block_height;
    }

    let lake_config = LakeConfig {
        s3_endpoint: PROJECT_CONFIG.s3_endpoint.clone(),
        s3_bucket_name: PROJECT_CONFIG.s3_bucket_name.clone(),
        s3_region_name: PROJECT_CONFIG.s3_region_name.clone(),
        start_block_height: current_height as u64,
    };
    Ok(lake_config)
}

pub fn init_env_config() -> Env {
    Env {
        s3_endpoint: None,
        s3_bucket_name: env::var("S3_BUCKET_NAME").unwrap(),
        s3_region_name: env::var("S3_REGION_NAME").unwrap(),
        start_block_height_from_cache: env::var("START_BLOCK_HEIGHT_FROM_CACHE")
            .unwrap()
            .parse::<bool>()
            .unwrap(),
        start_block_height: env::var("START_BLOCK_HEIGHT")
            .unwrap()
            .parse::<i64>()
            .unwrap(),
        push_engine: env::var("PUSH_ENGINE").unwrap().parse::<bool>().unwrap(),
        push_engine_url: env::var("PUSH_ENGINE_URL").unwrap(),
        http_server_listen: env::var("HTTP_SERVER_LISTEN").unwrap(),
        database_url: env::var("DATABASE_URL").unwrap(),
    }
}

pub async fn init_db_pool() {
    let pool = PgPoolOptions::new()
        .connect(&PROJECT_CONFIG.database_url)
        .await
        .unwrap();
    DB_POOL.set(pool).unwrap();
}
