Use [ledger_app_builder](https://github.com/LedgerHQ/ledger-app-builder) Docker container

Prerequisite:
```
# Pull Docker image container
docker pull ghcr.io/ledgerhq/ledger-app-builder/ledger-app-builder
# Checkout LedgerHQ Starknet app repository
git clone https://github.com/LedgerHQ/app-starknet.git
```

Build for Nano S+/X/Stax/Flex:
```
docker run --rm -it -v "$(pwd -P):/apps" ghcr.io/ledgerhq/ledger-app-builder/ledger-app-builder
cd /apps/app-starknet/starknet
cargo clean
cargo ledger build nanosplus|nanox|stax|flex
```