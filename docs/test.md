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
## Launch Nano application in Speculos env
```
git clone https://github.com/LedgerHQ/nano-rapp-starknet.git
cd nano-rapp-starknet
```
### Nano S
```
docker run --rm -it -v $(pwd)/target/nanos/release:/speculos/apps --publish 5001:5001 --publish 9999:9999 speculos-runner --display headless --api-port 5001 --apdu-port 9999 --model nanos apps/nano-rapp-starknet
```
### Nano S+
```
docker run --rm -it -v $(pwd)/target/nanosplus/release:/speculos/apps --publish 5001:5001 --publish 9999:9999 speculos-runner --display headless --api-port 5001 --apdu-port 9999 --model nanosp --apiLevel 1 apps/nano-rapp-starknet
```
### Nano X
```
docker run --rm -it -v $(pwd)/target/nanox/release:/speculos/apps --publish 5001:5001 --publish 9999:9999 speculos-runner --display headless --api-port 5001 --apdu-port 9999 --model nanox --apiLevel 1 apps/nano-rapp-starknet
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