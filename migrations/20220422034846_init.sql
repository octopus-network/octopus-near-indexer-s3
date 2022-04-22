CREATE TABLE IF NOT EXISTS "indexer_raw"
(
    "hash"      text  PRIMARY KEY,
    "prev_hash" text  NOT NULL,
    "height"    numeric NOT NULL,
    "raw"       jsonb NOT NULL
);

