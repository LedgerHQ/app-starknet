Use [ledger_app_builder](https://github.com/LedgerHQ/ledger-app-builder) Docker container

Prerequisite:
```
# Pull Docker image container
docker pull ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools
# Checkout LedgerHQ Starknet app repository
git clone https://github.com/LedgerHQ/app-starknet.git
```

Build for Nano S+/X/Stax/Flex:
```
docker run --rm -it -v "$(pwd -P):/apps" --publish 5001:5001 --publish 9999:9999 -e DISPLAY='host.docker.internal:0' -v '/tmp/.X11-unix:/tmp/.X11-unix' --privileged ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools
cd /apps/app-starknet/starknet
cargo clean
cargo ledger build nanosplus|nanox|stax|flex
```