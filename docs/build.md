Use [ledger_app_builder](https://github.com/yogh333/ledger-app-builder/tree/ledger-starknet-app-builder)

```
docker build -t starknet-app-builder:latest .
docker run --rm -ti -v $(pwd)/app:/app starknet-app-builder:latest
cd cargo-ledger
cargo install --path .
cd ../nano-app-starknet/
cargo ledger nanos -- --message-format=json
```