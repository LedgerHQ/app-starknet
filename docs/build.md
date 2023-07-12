### Set Docker env
```
git clone https://github.com/yogh333/ledger-app-builder.git
cd ledger-app-builder
git checkout ledger-starknet-app-builder
docker build -t starknet-builder:latest -f legacy/Dockerfile .
cd ..
```

### Checkout Nano app and plugin repositories (for instance erc-20 plugin)
```
mkdir apps
cd apps
git clone https://github.com/LedgerHQ/nano-rapp-starknet.git
git checkout feat/plugin_call
git clone https://github.com/LedgerHQ/plugin-erc20.git rapp-plugin-erc20
```

### Build
```
docker run --rm -ti -v $(pwd)/apps:/app starknet-builder:latest
export BOLOS_SDK=/opt/nanos-secure-sdk|nanosplus-secure-sdk|nanox-secure-sdk
cd nano-rapp-starknet/
cago clean
cargo ledger build nanos|nanosplus|nanox
cd ../plugin-erc20
cago clean
cargo ledger build nanos|nanosplus|nanox
```
