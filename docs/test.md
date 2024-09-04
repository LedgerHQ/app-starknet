Use [ledger_app_builder](https://github.com/LedgerHQ/ledger-app-builder) Docker container

Prerequisite:
```
# Get Docker container
docker pull ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools
```
`
### Run Ragger tests
```
docker run --rm -it -v "$(pwd -P):/apps" --publish 5001:5001 --publish 9999:9999 -e DISPLAY='host.docker.internal:0' -v '/tmp/.X11-unix:/tmp/.X11-unix' --privileged ghcr.io/ledgerhq/ledger-app-builder/ledger-app-dev-tools
cd /apps/app-starknet
pip install -r tests/requirements.txt 
pytest tests/ --tb=short -v --device {nanosp | nanox | stax | flex}
```

### Emulator
From the Docker container, you can also run the app directly with the [Speculos](https://github.com/LedgerHQ/speculos) emulator.
For instance, for Nano S+:
```
speculos -m nanosp --apdu-port 9999 --api-port 5001 --display headless target/nanosplus/release/starknet
```
or for Stax:
```
speculos -m stax --apdu-port 9999 --api-port 5001 target/stax/release/starknet
```

# Device
Use [ledgerctl](https://github.com/LedgerHQ/ledgerctl) to install the app on a Nano device. For instance, for Nano S+:
```
ledgerctl install -f app_nanosplus.json
```
Use [ledgercomm](https://github.com/LedgerHQ/ledgercomm) to send APDU e.g :
```
ledgercomm-send --hid file tools/apdu-generator/apdu_samples/get_pub_key_confirm.dat
```