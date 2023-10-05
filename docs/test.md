Use [ledger_app_builder](https://github.com/LedgerHQ/ledger-app-builder)

```
# Get Docker container
git clone https://github.com/LedgerHQ/ledger-app-builder
cd ledger-app-builder
docker build -t ledger-app-builder:latest -f dev-tools/Dockerfile .

### Nano S
```
docker run --rm -it -v $(pwd)/target/nanos/release:/speculos/apps --publish 5001:5001 --publish 9999:9999 ledger-app-builder:latest --display headless --api-port 5001 --apdu-port 9999 --model nanos apps/nano-rapp-starknet
```
### Nano S+
```
docker run --rm -it -v $(pwd)/target/nanosplus/release:/speculos/apps --publish 5001:5001 --publish 9999:9999 ledger-app-builder:latest --display headless --api-port 5001 --apdu-port 9999 --model nanosp --apiLevel 1 apps/nano-rapp-starknet
```
### Nano X
```
docker run --rm -it -v $(pwd)/target/nanox/release:/speculos/apps --publish 5001:5001 --publish 9999:9999 ledger-app-builder:latest --display headless --api-port 5001 --apdu-port 9999 --model nanox --apiLevel 1 apps/nano-rapp-starknet
```
Use [ledgercomm](https://github.com/LedgerHQ/ledgercomm) to send APDU e.g :
```
ledgercomm-send file test/sign.apdu
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