Use [ledger_app_builder](https://github.com/LedgerHQ/ledger-app-builder)

```
# Get Docker container
git clone https://github.com/LedgerHQ/ledger-app-builder
cd ledger-app-builder
docker build -t ledger-app-builder:full -f full/Dockerfile .
# Checkout Starknet Nano app repository
mkdir app
cd app
git clone https://github.com/LedgerHQ/nano-rapp-starknet.git
```

Build:
```
docker run --rm -ti -v $(pwd)/app:/app ledger-app-builder:full
cargo ledger setup
cd nano-rapp-starknet/
cargo clean
cargo ledger build nanos|nanosplus|nanox
```


Build for Nano device:
```
docker run --rm -ti -v $(pwd)/app:/app ledger-app-builder:full
cargo ledger setup
cd nano-rapp-starknet/
cargo ledger build nanos|nanosplus|nanox -- --no-default-features --features device
```