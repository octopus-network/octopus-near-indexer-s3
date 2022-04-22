sea_query::sea_query_driver_postgres!();

use crate::db_pool;
use anyhow::Result;
use sea_query::{Iden, PostgresQueryBuilder, Query, Values, OnConflict};
use sea_query_driver_postgres::bind_query;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Iden)]
pub enum IndexerRaw {
    Table,
    Hash,
    PrevHash,
    Height,
    Raw,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct RawTableStruct {
    pub prev_hash: String,
    pub height: u64,
    pub hash: String,
    pub raw: Value,
}

impl RawTableStruct {
    pub fn build_insert(self) -> (String, Values) {
        let mut query = Query::insert().into_table(IndexerRaw::Table).to_owned();
        query.columns(vec![
            IndexerRaw::PrevHash,
            IndexerRaw::Height,
            IndexerRaw::Hash,
            IndexerRaw::Raw,
        ]);
        query
            .values(vec![
                self.prev_hash.into(),
                self.height.into(),
                self.hash.into(),
                self.raw.into(),
            ])
            .expect("DB query data fail");
        query.on_conflict(
            OnConflict::column(IndexerRaw::Hash)
                .do_nothing()
                .to_owned(),
        );
        query.build(PostgresQueryBuilder)
    }
}

impl IndexerRaw {
    pub async fn fetch(raw: RawTableStruct) -> Result<()> {
        let (sql, values) = RawTableStruct::build_insert(raw);
        let _row = bind_query(sqlx::query(&sql), &values)
            .fetch_all(db_pool())
            .await?;
        Ok(())
    }
}
