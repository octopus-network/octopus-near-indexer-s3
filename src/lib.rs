use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres};

pub mod cache;
pub mod config;
pub mod indexer;
pub mod server;

pub const INDEXER: &str = "octopus-near-indexer-s3";
pub const HTTP_INDEXER: &str = "http";

pub static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub fn db_pool() -> &'static Pool<Postgres> {
    DB_POOL.get().unwrap()
}
