Use [ledger_starknet_app_builder](https://github.com/yogh333/ledger-app-builder/tree/ledger-starknet-app-builder)

```
# Get Docker env
git clone https://github.com/yogh333/ledger-app-builder.git
cd ledger-app-builder
git checkout -b ledger-starknet-app-builder origin/ledger-starknet-app-builder
docker build -t ledger-starknet-app-builder:latest .
# Checkout 3 Rust repositories (UI, SDK, Starknet app)
mkdir app
cd app
git clone https://github.com/yogh333/ledger-nanos-ui.git
cd ledger-nanos-ui
git checkout -b speculos origin/speculos
cd ..
git clone https://github.com/yogh333/ledger-nanos-sdk.git
cd ledger-nanos-sdk
git checkout -b ecc-pubkey-getter origin/ecc-pubkey-getter
cd ..
git clone https://github.com/LedgerHQ/nano-rapp-starknet.git
```

Build for Speculos env:
```
docker run --rm -ti -v $(pwd)/app:/app ledger-starknet-app-builder:latest
cd app/nano-app-starknet/
cargo ledger nanosplus
```


Build for Nano device:
```
docker run --rm -ti -v $(pwd)/app:/app ledger-starknet-app-builder:latest
cd app/nano-app-starknet/
cargo ledger nanosplus -- --no-default-features --features device
```