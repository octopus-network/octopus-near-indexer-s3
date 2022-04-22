sea_query::sea_query_driver_postgres!();

use crate::db_pool;
use anyhow::Result;
use sea_query::{Expr, Iden, OnConflict, Order, PostgresQueryBuilder, Query, Values};
use sea_query_driver_postgres::bind_query;
use sea_query_driver_postgres::bind_query_as;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Iden)]
pub enum IndexerRawTable {
    Table,
    Hash,
    PrevHash,
    Height,
    Raw,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct RawTableStruct {
    pub prev_hash: String,
    pub height: i64,
    pub hash: String,
    pub raw: Value,
}

impl RawTableStruct {
    pub fn build_insert(self) -> (String, Values) {
        let mut query = Query::insert()
            .into_table(IndexerRawTable::Table)
            .to_owned();
        query.columns(vec![
            IndexerRawTable::PrevHash,
            IndexerRawTable::Height,
            IndexerRawTable::Hash,
            IndexerRawTable::Raw,
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
            OnConflict::column(IndexerRawTable::Hash)
                .do_nothing()
                .to_owned(),
        );
        query.build(PostgresQueryBuilder)
    }

    pub fn build_select_from_height_to_current() -> (String, Values) {
        let mut query = Query::select()
            .from(IndexerRawTable::Table)
            .expr(Expr::asterisk())
            .to_owned();
        query.order_by(IndexerRawTable::Height, Order::Desc);
        query.limit(1);
        query.build(PostgresQueryBuilder)
    }
}

impl IndexerRawTable {
    pub async fn insert(raw: RawTableStruct) -> Result<()> {
        let (sql, values) = RawTableStruct::build_insert(raw);
        let _row = bind_query(sqlx::query(&sql), &values)
            .fetch_all(db_pool())
            .await?;
        Ok(())
    }

    pub async fn select_current_height() -> Result<RawTableStruct> {
        let (sql, values) = RawTableStruct::build_select_from_height_to_current();
        let row = bind_query_as(sqlx::query_as::<_, RawTableStruct>(&sql), &values)
            .fetch_one(db_pool())
            .await?;
        Ok(row)
    }
}
