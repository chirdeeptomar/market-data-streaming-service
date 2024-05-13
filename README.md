# Sample GRPC streaming server

1) Run Kafka or Redpanda

```docker
cd docker && docker-compose up;
```

2) Run the publisher

```sh
cargo run --bin market-data-publisher
```

3) Run the server

```sh
cargo run --bin market-data-server
```
