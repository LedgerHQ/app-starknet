Use [ledger_app_builder](https://github.com/LedgerHQ/ledger-app-builder) Docker container

Prerequisite:
```
# Pull Docker image container
docker pull ghcr.io/ledgerhq/ledger-app-builder/ledger-app-builder
# Checkout LedgerHQ Starknet app repository
git clone https://github.com/LedgerHQ/app-starknet.git
```

Build for Nano S/S+/X:
```
docker run --rm -it -v "$(pwd -P):/apps" ghcr.io/ledgerhq ledger-app-builder/ledger-app-builder
cd /apps/app-starknet/
cargo clean
cargo ledger build nanos|nanosplus|nanox
```