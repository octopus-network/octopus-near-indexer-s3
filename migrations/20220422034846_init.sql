CREATE TABLE IF NOT EXISTS "indexer_raw_table"
(
    "hash"      text  PRIMARY KEY,
    "prev_hash" text  NOT NULL,
    "height"    bigint NOT NULL,
    "raw"       jsonb NOT NULL
);

