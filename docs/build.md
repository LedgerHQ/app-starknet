Use [ledger_starknet_app_builder](https://github.com/yogh333/ledger-app-builder/tree/ledger-starknet-app-builder)

```
# Get Docker env
git clone https://github.com/yogh333/ledger-app-builder.git
cd ledger-app-builder
git checkout -b ledger-starknet-app-builder origin/ledger-starknet-app-builder
docker build -t ledger-starknet-app-builder:latest -f legacy/Dockerfile .

# Checkout Nano app and plugin repositories
mkdir app
cd app
git clone https://github.com/LedgerHQ/nano-rapp-starknet.git
git checkout -b feat/plugin_call origin/feat/plugin_call
git clone https://github.com/LedgerHQ/plugin-erc20.git rapp-plugin-erc20
```

Build for Speculos env:
```
docker run --rm -ti -v $(pwd)/app:/app ledger-starknet-app-builder:latest
export BOLOS_SDK=/opt/nanos-secure-sdk|nanosplus-secure-sdk|nanox-secure-sdk
cd nano-rapp-starknet/
cago clean
cargo ledger build nanos|nanosplus|nanox
cd ../plugin-erc20
cago clean
cargo ledger build nanos|nanosplus|nanox
```


Build for Nano device:
```
docker run --rm -ti -v $(pwd)/app:/app ledger-starknet-app-builder:latest
export BOLOS_SDK=/opt/nanos-secure-sdk|nanosplus-secure-sdk|nanox-secure-sdk
cd nano-rapp-starknet/
cargo clean
cargo ledger build nanos|nanosplus|nanox -- --no-default-features --features device
cd ../plugin-erc20
cago clean
cargo ledger build nanos|nanosplus|nanox -- --no-default-features --features device
```