CREATE TABLE IF NOT EXISTS "near_indexer_raw"
(
    "hash"      text      NOT NULL,
    "prev_hash" text      NOT NULL,
    "height"    bigint    NOT NULL,
    "raw"       jsonb     NOT NULL,
    "date"      timestamp NOT NULL,
    PRIMARY KEY (hash, date)
) PARTITION BY RANGE (date);

CREATE TABLE IF NOT EXISTS near_indexer_raw_old PARTITION OF near_indexer_raw
    FOR VALUES FROM ('2020-01-01') TO ('2022-05-01');

CREATE TABLE IF NOT EXISTS near_indexer_raw_202205 PARTITION OF near_indexer_raw
    FOR VALUES FROM ('2022-05-01') TO ('2022-06-01');

CREATE TABLE IF NOT EXISTS near_indexer_raw_202206 PARTITION OF near_indexer_raw
    FOR VALUES FROM ('2022-06-01') TO ('2022-07-01');

CREATE TABLE IF NOT EXISTS near_indexer_raw_202207 PARTITION OF near_indexer_raw
    FOR VALUES FROM ('2022-07-01') TO ('2022-08-01');