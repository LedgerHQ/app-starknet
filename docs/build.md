Use [ledger_starknet_app_builder](https://github.com/yogh333/ledger-app-builder/tree/ledger-starknet-app-builder)

```
docker build -t ledger-starknet-app-builder:latest .
docker run --rm -ti -v $(pwd)/app:/app ledger-starknet-app-builder:latest
cd app/nano-app-starknet/
```

For Speculos:
```
cargo ledger nanosplus
```


For a Nano device:
```
cargo ledger nanosplus -- --no-default-features --features device
```