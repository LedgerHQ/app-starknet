# Functional tests

> :point_right: Every path on this document assumes you are at the root of the repository

`tests/ledgercomm/` contains functional tests which use the
  [Python LedgerComm library](https://github.com/LedgerHQ/ledgercomm), which allows the tests to run either on an actual Nano, or on [Speculos](https://github.com/LedgerHQ/speculos)

  Before running the tests you will need to setup a Python virtual env and install all the modules listed in `tests/ledgercomm/requirements.txt`

  To install `cairo-lang` module, please check [Setting up the Cairo env](https://starknet.io/docs/quickstart.html#quickstart)

  Also note that a on Mac M1, check [here](https://github.com/starkware-libs/cairo-lang/issues/68) to install `ecdsa`, `fastecdsa` and `sympy`
