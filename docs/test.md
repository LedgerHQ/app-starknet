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
```
cd ledger-app-builder
```
### Nano S
```
docker run --rm -it -v $(pwd)/app:/speculos/apps --publish 5001:5001 --publish 9999:9999 speculos-runner --display headless --api-port 5001 --apdu-port 9999 --model nanos -l plugin-erc20:apps/rapp-plugin-erc20/target/nanos/release/plugin-erc20 apps/nano-rapp-starknet/target/nanos/release/nano-rapp-starknet
```
### Nano S+
```
docker run --rm -it -v $(pwd)/app:/speculos/apps --publish 5001:5001 --publish 9999:9999 speculos-runner --display headless --api-port 5001 --apdu-port 9999 --model nanosp --apiLevel 1 -l plugin-erc20:apps/rapp-plugin-erc20/target/nanosplus/release/plugin-erc20 apps/nano-rapp-starknet/target/nanosplus/release/nano-rapp-starknet
```
### Nano X
```
docker run --rm -it -v $(pwd)/app:/speculos/apps --publish 5001:5001 --publish 9999:9999 speculos-runner --display headless --api-port 5001 --apdu-port 9999 --model nanox --apiLevel 1 -l plugin-erc20:apps/rapp-plugin-erc20/target/nanox/release/plugin-erc20 apps/nano-rapp-starknet/target/nanox/release/nano-rapp-starknet
```
Use [cargo ledger](https://github.com/yogh333/cargo-ledger/tree/feat/device_testing) to send APDU e.g :
```
cargo ledger send apdu.dat
```

# Device
Use [ledgerctl](https://github.com/LedgerHQ/ledgerctl) to install the app on a Nano device. For instance, for Nano S+:
```
git clone https://github.com/LedgerHQ/nano-rapp-starknet.git
cd nano-rapp-starknet/release/nanosplus
ledgerctl install -f app_nanosplus.json
```
Use [ledgercomm](https://github.com/LedgerHQ/ledgercomm) to send APDU e.g :
```
cd nano-rapp-starknet/
ledgercomm-send --hid file test/sign.apdu
```