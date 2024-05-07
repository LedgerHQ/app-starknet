Use [ledger_app_builder](https://github.com/LedgerHQ/ledger-app-builder) Docker container

Prerequisite:
```
# Get Docker container
docker pull ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools
```
`
### Run Ragger tests
```
docker run --rm -it -v "$(pwd -P):/apps" --publish 5001:5001 --publish 9999:9999 ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools
cd /apps/app-starknet
pip install -r tests/requirements.txt 
pytest tests/ --tb=short -v --device {nanos | nanosp | nanox}
```

### Emulator
From the Docker container, you can also run the app directly with the [Speculos](https://github.com/LedgerHQ/speculos) emulator.
For instance, for Nano S+:
```
speculos -m nanosp --apdu-port 9999 --api-port 5001 target/nanosplus/release/starknet
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