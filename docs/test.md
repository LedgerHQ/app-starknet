# Speculos
## Install speculos
Use [speculos](https://github.com/LedgerHQ/speculos)
```
git clone https://github.com/LedgerHQ/speculos.git
cd speculos
docker build -t speculos-runner:latest .
```
## Launch Nano application in Speculos env
```
git clone https://github.com/LedgerHQ/nano-rapp-starknet.git
cd nano-rapp-starknet
docker run --rm -it -v $(pwd)/target/nanosplus/release:/speculos/apps --publish 5001:5001 --publish 9999:9999 speculos-runner --display headless --api-port 5001 --apdu-port 9999 --model nanosp --sdk 1.0.3 apps/nano-rapp-starknet
```
Use [ledgercomm](https://github.com/LedgerHQ/ledgercomm) to send APDU e.g :
```
ledgercomm-send file test/sign.apdu
```

# Device
Use [ledgerctl](https://github.com/LedgerHQ/ledgerctl) to install the app on a Nano device
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