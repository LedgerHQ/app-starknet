# Speculos
## Install speculos
Use [speculos](https://github.com/LedgerHQ/speculos)
```
git clone https://github.com/LedgerHQ/speculos.git
cd speculos
docker build -t speculos-builder:latest -f build.Dockerfile .
```
Edit Dockerfile and modify the first line as following:
```
FROM speculos-builder:latest AS builder
```
Then
```
docker build -t speculos-runner:latest .
```
## Launch Starknet application with erc-20 plugin in Speculos env
### Nano S
```
docker run --rm -it -v $(pwd)/apps:/speculos/apps --publish 5001:5001 --publish 9999:9999 speculos-runner --display headless --api-port 5001 --apdu-port 9999 --model nanos -l plugin-erc20:apps/rapp-plugin-erc20/target/nanos/release/plugin-erc20 apps/nano-rapp-starknet/target/nanos/release/nano-rapp-starknet
```
### Nano S+
```
docker run --rm -it -v $(pwd)/apps:/speculos/apps --publish 5001:5001 --publish 9999:9999 speculos-runner --display headless --api-port 5001 --apdu-port 9999 --model nanosp --apiLevel 1 -l plugin-erc20:apps/rapp-plugin-erc20/target/nanosplus/release/plugin-erc20 apps/nano-rapp-starknet/target/nanosplus/release/nano-rapp-starknet
```
### Nano X
```
docker run --rm -it -v $(pwd)/apps:/speculos/apps --publish 5001:5001 --publish 9999:9999 speculos-runner --display headless --api-port 5001 --apdu-port 9999 --model nanox --apiLevel 1 -l plugin-erc20:apps/rapp-plugin-erc20/target/nanox/release/plugin-erc20 apps/nano-rapp-starknet/target/nanox/release/nano-rapp-starknet
```
Use [starknet-apdu-generator](https://github.com/LedgerHQ/starknet-apdu-generator) to generate APDUs from a Tx, e.g
```
cargo run --bin starknet-apdu-generator -- --json transaction_starknetid_bmc.json
```
Use [rust-ledger](https://github.com/yogh333/rust-ledger/tree/feat/apdu_stream) to send APDU from `apdu.json` created by `starknet-apdu-generator` e.g :
```
cargo run -p ledger-cli -- --filters tcp --timeout 30s file apdu.json
```

# Device
Use [ledgerctl](https://github.com/LedgerHQ/ledgerctl) to install the app on a Nano device. For instance, for Nano S+:
```
cd nano-rapp-starknet
ledgerctl install -f app_nanosplus.json
cd plugin-erc20
ledgerctl install -f app_nanosplus.json
```
Use [rust-ledger](https://github.com/yogh333/rust-ledger/tree/feat/apdu_stream) to send APDU from `apdu.json` created by `starknet-apdu-generator` e.g :
```
cargo run -p ledger-cli -- --filters hid --timeout 30s file apdu.json
```
