## octopus-near-indexer-s3

![](./doc/images/main.svg)

## Doc

### Cache
Indexer S3 data cache to PostgresDB or RocksDB and Redis.
PostgresDB or RocksDB cache Indexer Raw.
Redis cache state

### Indexer
Get S3 data and tracking new block. Based NEAR Lake.

### Pusher
Http Client post S3 data and state event to url path.

### Server
Http Server API to use cache and S3 data. Provide RESTful API and GraphQL.

Get `HTTP_SERVER_LISTEN/cache` current sync height block data.

Get `HTTP_SERVER_LISTEN/cache/<id>` custom height block data.


## Roadmap

#### Cache

- [ ] Local cache use RocksDB
- [ ] DB fail Retry

#### Indexer

- [x] From Cache
- [ ] Network error retry

#### Pusher

- [x] Websocket push (graphql-engine)

#### Server

- [x] Add more api (graphql-engine)
- [x] GraphQL (graphql-engine)



## How to Run

### AWS S3 Credentials

In order to be able to get objects from the AWS S3 bucket you need to provide the AWS credentials.

AWS default profile configuration with aws configure looks similar to the following:

`~/.aws/credentials`
```
[default]
aws_access_key_id=
aws_secret_access_key=
```

[AWS docs: Configuration and credential file settings](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)

### Env Config

```
// This is S3 config
S3_BUCKET_NAME=near-lake-data-mainnet
S3_REGION_NAME=eu-central-1

// Start from cache if true ignore START_BLOCK_HEIGHT
START_BLOCK_HEIGHT_FROM_CACHE=true
START_BLOCK_HEIGHT=0

// PUSH block and event to Engine if false ignore PUSH_ENGINE_URL
ENABLE_PUSH_SERVER=false
PUSH_URL=http://127.0.0.1:8088

// Http Server
HTTP_SERVER_LISTEN=127.0.0.1:8088

// Cache DB
DATABASE_URL=postgres://nearscan_raw:nearscan_raw@192.168.20.40/nearscan
```


### Run
```shell
cargo build --release
./target/release/octopus-near-indexer-s3
```