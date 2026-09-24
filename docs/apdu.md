# APDU protocol description

This document aims to provide a description of the APDU protocol supported by the app, explaining what each instruction does, the expected parameters and return values

## General Structure

The general structure of a request and response is as followed:

### Request / Command

| Field   | Type     | Content                | Note                   |
|:--------|:---------|:-----------------------|------------------------|
| CLA     | byte (1) | Application Identifier | 0x5A                   |
| INS     | byte (1) | Instruction ID         |                        |
| P1      | byte (1) | Parameter 1            |                        |
| P2      | byte (1) | Parameter 2            |                        |
| L       | byte (1) | Bytes in payload       |                        |
| PAYLOAD | byte (L) | Payload                |                        |

### Response

| Field   | Type     | Content     | Note                     |
| ------- | -------- | ----------- | ------------------------ |
| ANSWER  | byte (N) | Answer      | depends on the command   |
| SW1-SW2 | byte (2) | Return code | see list of return codes |

#### Return codes

| Return code | Description             |
| ----------- | ----------------------- |
| 0x9000      | Success                 |
| 0x68xx      | Syscall Error           |
| 0x6982      | Empty buffer            |
| 0x6e00      | Bad Cla                 |
| 0x6e01      | Bad Ins                 |
| 0x6e02      | Bad P1/P2               |
| 0x6e03      | Bad Len                 |
| 0x6e04      | User Cancelled          |
| 0xe000      | Panic                   |
| 0xff03      | Bad state: APDU out of sequence, or inconsistent with the transaction sent so far. The signing request is dropped |
| 0xff04      | Invalid fees: the fee APDU does not hold a tip and resource bounds in their wire format. The signing request is dropped |
| 0xff05      | Contract address mismatch: the Deploy Account address is not the one derived from the class hash, salt and constructor calldata. The signing request is dropped |
| 0xff06      | Too many constructor arguments: more than 8 constructor calldata. The signing request is dropped |
| 0xff07      | Too many calls: a call beyond the announced number of calls. The signing request is dropped |

### Transaction signing sequence

The four transaction-signing instructions (INS 0x03 to 0x06) stream a transaction over several APDUs:

- `P1 = 0x00` starts a new request and resets any request in progress.
- Every following P1 is sent **once and in order** (`0x01`, `0x02`, ...). Only the last P1 of the instruction repeats, for as long as it streams calls or constructor calldata.
- Any other APDU (a skipped or replayed stage, or a stage of another instruction) is refused with `0xff03`, and so is any payload the app cannot accept. Either way the request is dropped: the host has to start again from `P1 = 0x00`.
- A request produces at most one signature. Once the transaction has been reviewed, whatever the outcome, the request is over.
- Counts (number of calls, number of calldata) are sent as a felt but must fit in 32 bits.


## Commands definitions

### GetVersion

This command will return the app version

#### Command

| Field | Type     | Content                | Expected |
|-------|----------|------------------------|----------|
| CLA   | byte (1) | Application Identifier | 0x5A     |
| INS   | byte (1) | Instruction ID         | 0x00     |
| P1    | byte (1) | Parameter 1            | ignored  |
| P2    | byte (1) | Parameter 2            | ignored  |
| L     | byte (1) | Bytes in payload       | 0        |

#### Response

| Field     | Type     | Content          | Note                            |
| --------- | -------- | ---------------- | ------------------------------- |
| MAJOR     | byte (1) | Version Major    |                                 |
| MINOR     | byte (1) | Version Minor    |                                 |
| PATCH     | byte (1) | Version Patch    |                                 |
| SW1-SW2   | byte (2) | Return code      | see list of return codes        |

### GetPubKey

This command returns the public key corresponding to the private key found at the given [EIP-2645](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-2645.md) path 

#### Command

| Field   | Type     | Content                   | Expected        |
|---------|----------|---------------------------|-----------------|
| CLA     | byte (1) | Application Identifier    | 0x5A            |
| INS     | byte (1) | Instruction ID            | 0x01            |
| P1      | byte (1) | Parameter 1               | if not 0, user will have to confirm          |
| P2      | byte (1) | Parameter 2               | ignored         |
| L       | byte (1) | Bytes in payload          | 0x18            |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55      |
| Path[1] | byte (4) | Derivation Path Data      |                 |
| Path[2] | byte (4) | Derivation Path Data      |                 |
| Path[3] | byte (4) | Derivation Path Data      |                 |
| Path[4] | byte (4) | Derivation Path Data      |                 |
| Path[5] | byte (4) | Derivation Path Data      |                 |

#### Response

| Field      | Type      | Content           | Note                     |
| ---------- | --------- | ----------------- | ------------------------ |
| PK_LEN     | byte (1)  | Bytes in PKEY     | 64                       |
| PKEY       | byte (64) | Public key bytes  | 32 (x) + 32 (y)          |
| SW1-SW2    | byte (2)  | Return code       | see list of return codes |

### Sign Hash

This command will return the signature of a Pedersen or Poseidon hash

#### Command #0: Set private key

| Field | Type     | Content                     | Expected          |
|-------|----------|-----------------------------|-------------------|
| CLA   | byte (1) | Application Identifier      | 0x5A              |
| INS   | byte (1) | Instruction ID              | 0x02              |
| P1    | byte (1) | Payload desc                | 0x00              |
| P2    | byte (1) | ignored                     |                   |
| L     | byte (1) | Bytes in payload            | (depends)         |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55        |
| Path[1] | byte (4) | Derivation Path Data      |                   |
| Path[2] | byte (4) | Derivation Path Data      |                   |
| Path[3] | byte (4) | Derivation Path Data      |                   |
| Path[4] | byte (4) | Derivation Path Data      |                   |
| Path[5] | byte (4) | Derivation Path Data      |                   |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #1: Send Hash

| Field | Type       | Content                     | Expected          |
|-------|------------|-----------------------------|-------------------|
| CLA   | byte (1)   | Application Identifier      | 0x5A              |
| INS   | byte (1)   | Instruction ID              | 0x02              |
| P1    | byte (1)   | Payload desc                | 0x01              |
| P2    | byte (1)   | ignored                     |                   |
| L     | byte (1)   | nb of bytes in payload      | 0x20              |
| Hash  | bytes (32) | hash                        |                   |

#### Response

| Field    | Type      | Content           | Note                                  |
|----------|-----------|-------------------|---------------------------------------|
| L        | byte (1)  | Sig Length        | 0x41 = 65                             |
| R        | byte (32) | Signature         | (R,S,V) encoded signature             |
| S        | byte (32) | Signature         | (R,S,V) encoded signature             |
| V        | byte (1)  | Signature         | (R,S,V) encoded signature             |
| SW1-SW2  | byte (2)  | Return code       | see list of return codes              |


### Sign INVOKE Tx v3 (see [Starnet Tx v3](https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v3_hash_calculation))

This command will return the hash and signature of a Starknet INVOKE Tx version 3

#### Command #0: Set private key

| Field | Type     | Content                     | Expected          |
|-------|----------|-----------------------------|-------------------|
| CLA   | byte (1) | Application Identifier      | 0x5A              |
| INS   | byte (1) | Instruction ID              | 0x03              |
| P1    | byte (1) | Payload desc                | 0x00              |
| P2    | byte (1) | ignored                     |                   |
| L     | byte (1) | Bytes in payload            | (depends)         |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55        |
| Path[1] | byte (4) | Derivation Path Data      |                   |
| Path[2] | byte (4) | Derivation Path Data      |                   |
| Path[3] | byte (4) | Derivation Path Data      |                   |
| Path[4] | byte (4) | Derivation Path Data      |                   |
| Path[5] | byte (4) | Derivation Path Data      |                   |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #1: Send INVOKE Tx fields

| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x03              |
| P1               | byte (1) | Payload desc                | 0x01              |
| P2               | byte (1) | ignored                     |                   |
| L                | byte (1) | Bytes in payload            | 0x80 (4x32 = 128) |
| SENDER ADDR      | byte (32)| sender address              | (depends)         |
| CHAIN_ID         | byte (32)| chain_id                    | (depends)         |
| NONCE            | byte (32)| nonce                       | (depends)         |
| DA MODE          | byte (32)| data_availability_mode      | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #2: Fees

| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x03              |
| P1               | byte (1) | Payload desc                | 0x02              |
| P2               | byte (1) | ignored                     | 0x00              |
| L                | byte (1) | Bytes in payload            | 0x60 or 0x80      |
| TIP              | byte (32)| tip                         | (depends)         |
| L1 FEE           | byte (32)| l1_gas_bounds               | (depends)         |
| L2 FEE           | byte (32)| l2_gas_bounds               | (depends)         |
| L1 DATA          | byte (32)| l1_data_gas_bounds (optional) | (depends)       |

The payload is 3 or 4 felts (`L` = 0x60 or 0x80): the tip, then the L1 gas, L2 gas and optional L1 data gas resource bounds. The tip must fit in a `u64`. Each resource bound packs, big-endian:

| Bytes  | Content                                                              |
|--------|----------------------------------------------------------------------|
| 0..8   | resource name, right-aligned: `L1_GAS`, `L2_GAS`, `L1_DATA`, in that order |
| 8..16  | max_amount (`u64`)                                                   |
| 16..32 | max_price_per_unit (`u128`)                                          |

Any other payload is refused with `0xff04`.

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #3: Send Paymaster data

| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x03              |
| P1               | byte (1) | Payload desc                | 0x03              |
| P2               | byte (1) | ignored                     | 0x00              |
| L                | byte (1) | Bytes in payload            | 0x00              |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #4: Send Account Deployment data

| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x03              |
| P1               | byte (1) | Payload desc                | 0x04              |
| P2               | byte (1) | ignored                     | 0x00              |
| L                | byte (1) | Bytes in payload            | 0x00              |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #5: Number of Calls

| Field            | Type       | Content                     | Expected          |
|------------------|------------|-----------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier      | 0x5A              |
| INS              | byte (1)   | Instruction ID              | 0x03              |
| P1               | byte (1)   | Payload desc                | 0x05              |
| P2               | byte (1)   | ignored                     | 0x00              |
| L                | byte (1)   | Bytes in payload            | 0x20              |
| Num of calls     | bytes (32) | Bytes in payload            | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #6: Call

Each call is sent as a `New Call` APDU (`P2 = 0x00`), followed by as many `Add calldata` APDUs (`P2 = 0x01`) as its calldata needs. The app refuses with `0xff03`:

- an `Add calldata` APDU when no call is in progress, or when the current call already has all its announced calldata;
- a `New Call` APDU while the current call still expects calldata;
- more calldata than announced in `NB CALLDATA`, a felt that is not canonical (`>= P`), or a payload that is not a whole number of felts;
- any other `P2` value.

A call beyond the announced number of calls is refused with `0xff07`.

##### New Call

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x03              |
| P1               | byte (1)   | Payload desc                                   | 0x06              |
| P2               | byte (1)   | New call                                       | 0x00              |
| L                | byte (1)   | Bytes in payload                               | (depends)         |
| TO               | bytes (32) | to                                             | (depends)         |
| SELECTOR         | bytes (32) | selector                                       | (depends)         |
| NB CALLDATA      | bytes (32) | nb_calldata                                    | (depends)         |
| calldata         | bytes (32) | calldata #0                                    | (depends)         |
| calldata         | bytes (32) | calldata #1                                    | (depends)         |
| calldata         | bytes (32) | calldata #2                                    | (depends)         |
| calldata         | bytes (32) | calldata #3                                    | (depends)         |

#### Response 

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

##### Add calldata to the current Call

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x03              |
| P1               | byte (1)   | Payload desc                                   | 0x06              |
| P2               | byte (1)   | Add calldata                                   | 0x01              |
| L                | byte (1)   | Bytes in payload                               | (depends)         |
| calldata         | bytes (32) | calldata #0                                    | (depends)         |
| calldata         | bytes (32) | calldata #1                                    | (depends)         |
| calldata         | bytes (32) | calldata #2                                    | (depends)         |
| calldata         | bytes (32) | calldata #3                                    | (depends)         |
| calldata         | bytes (32) | calldata #4                                    | (depends)         |
| calldata         | bytes (32) | calldata #5                                    | (depends)         |
| calldata         | bytes (32) | calldata #6                                    | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |


#### Response (when Tx is complete)

| Field    | Type      | Content           | Note                                  |
|----------|-----------|-------------------|---------------------------------------|
| Tx Hash  | byte (32) | Tx Poseidon Hash  | 32 bytes                              |
| L        | byte (1)  | Sig Length        | 0x41 = 65                             |
| R        | byte (32) | Signature         | (R,S,V) encoded signature             |
| S        | byte (32) | Signature         | (R,S,V) encoded signature             |
| V        | byte (1)  | Signature         | (R,S,V) encoded signature             |
| SW1-SW2  | byte (2)  | Return code       | see list of return codes              |

### Sign Deploy Account Tx v3 (see [Starnet Deploy v3](https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v3_hash_calculation_3))

This command will return the hash and signature of a Starknet DEPLOY_ACCOUNT Tx version 3

#### Command #0: Set private key

| Field | Type     | Content                     | Expected          |
|-------|----------|-----------------------------|-------------------|
| CLA   | byte (1) | Application Identifier      | 0x5A              |
| INS   | byte (1) | Instruction ID              | 0x05              |
| P1    | byte (1) | Payload desc                | 0x00              |
| P2    | byte (1) | ignored                     |                   |
| L     | byte (1) | Bytes in payload            | (depends)         |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55        |
| Path[1] | byte (4) | Derivation Path Data      |                   |
| Path[2] | byte (4) | Derivation Path Data      |                   |
| Path[3] | byte (4) | Derivation Path Data      |                   |
| Path[4] | byte (4) | Derivation Path Data      |                   |
| Path[5] | byte (4) | Derivation Path Data      |                   |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #1: Send DEPLOY_ACCOUNT Tx fields

All six fields must be canonical felts (`< P`); anything else is refused with `0xff03`. `CONTRACT ADDR` must be the address Starknet derives from `CLASS HASH`, `SALT` and the constructor calldata with the zero deployer: the app recomputes it once the transaction is complete and refuses a mismatch with `0xff05`. The review shows the address, the fees, the class hash, the salt and every constructor argument.


| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x05              |
| P1               | byte (1) | Payload desc                | 0x01              |
| P2               | byte (1) | ignored                     |                   |
| L                | byte (1) | Bytes in payload            | 0xC0              |
| CONTRACT ADDR    | byte (32)| contract_address            | (depends)         |
| CHAIN_ID         | byte (32)| chain_id                    | (depends)         |
| NONCE            | byte (32)| nonce                       | (depends)         |
| DA MODE          | byte (32)| data_availability_mode      | (depends)         |
| CLASS HASH       | byte (32)| class_hash                  | (depends)         |
| SALT             | byte (32)| contract_address_salt       | (depends)         |


#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #2: Fees

| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x05              |
| P1               | byte (1) | Payload desc                | 0x02              |
| P2               | byte (1) | ignored                     | 0x00              |
| L                | byte (1) | Bytes in payload            | 0x60 or 0x80      |
| TIP              | byte (32)| tip                         | (depends)         |
| L1 FEE           | byte (32)| l1_gas_bounds               | (depends)         |
| L2 FEE           | byte (32)| l2_gas_bounds               | (depends)         |
| L1 DATA          | byte (32)| l1_data_gas_bounds (optional) | (depends)       |

The payload is 3 or 4 felts (`L` = 0x60 or 0x80): the tip, then the L1 gas, L2 gas and optional L1 data gas resource bounds. The tip must fit in a `u64`. Each resource bound packs, big-endian:

| Bytes  | Content                                                              |
|--------|----------------------------------------------------------------------|
| 0..8   | resource name, right-aligned: `L1_GAS`, `L2_GAS`, `L1_DATA`, in that order |
| 8..16  | max_amount (`u64`)                                                   |
| 16..32 | max_price_per_unit (`u128`)                                          |

Any other payload is refused with `0xff04`.

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #3: Send Paymaster data

| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x05              |
| P1               | byte (1) | Payload desc                | 0x03              |
| P2               | byte (1) | ignored                     | 0x00              |
| L                | byte (1) | Bytes in payload            | 0x00              |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |


#### Command #4: Number of constructor calldata

At most 8 constructor calldata are accepted, since every one of them is shown on screen; more is refused with `0xff06`.

| Field            | Type       | Content                     | Expected          |
|------------------|------------|-----------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier      | 0x5A              |
| INS              | byte (1)   | Instruction ID              | 0x05              |
| P1               | byte (1)   | Payload desc                | 0x04              |
| P2               | byte (1)   | ignored                     | 0x00              |
| L                | byte (1)   | Bytes in payload            | 0x20              |
| Num of calldata  | bytes (32) | Bytes in payload            | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #5: Constructor calldata

Chunks of at most 7 felts, until the announced number of calldata is reached. A constructor without arguments is sent as a single empty chunk (`L` = 0). The app refuses with `0xff03` more calldata than announced, a non-canonical felt, or an empty chunk while calldata is still expected.

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x05              |
| P1               | byte (1)   | Payload desc                                   | 0x05              |
| P2               | byte (1)   | ignored                                        |                   |
| L                | byte (1)   | Bytes in payload                               | (depends)         |
| calldata         | bytes (32) | calldata #0                                    | (depends)         |
| calldata         | bytes (32) | calldata #1                                    | (depends)         |
| calldata         | bytes (32) | calldata #2                                    | (depends)         |
| calldata         | bytes (32) | calldata #3                                    | (depends)         |
| calldata         | bytes (32) | calldata #4                                    | (depends)         |
| calldata         | bytes (32) | calldata #5                                    | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Response (when Tx is complete)

| Field    | Type      | Content           | Note                                  |
|----------|-----------|-------------------|---------------------------------------|
| Tx Hash  | byte (32) | Tx Poseidon Hash  | 32 bytes                              |
| L        | byte (1)  | Sig Length        | 0x41 = 65                             |
| R        | byte (32) | Signature         | (R,S,V) encoded signature             |
| S        | byte (32) | Signature         | (R,S,V) encoded signature             |
| V        | byte (1)  | Signature         | (R,S,V) encoded signature             |
| SW1-SW2  | byte (2)  | Return code       | see list of return codes              |


### Sign INVOKE Tx v1 (see [Starnet Tx v1](https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v1_deprecated_hash_calculation))

This command will return the hash and signature of a Starknet INVOKE Tx version 1

#### Command #0: Set private key

| Field | Type     | Content                     | Expected          |
|-------|----------|-----------------------------|-------------------|
| CLA   | byte (1) | Application Identifier      | 0x5A              |
| INS   | byte (1) | Instruction ID              | 0x04              |
| P1    | byte (1) | Payload desc                | 0x00              |
| P2    | byte (1) | ignored                     |                   |
| L     | byte (1) | Bytes in payload            | (depends)         |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55        |
| Path[1] | byte (4) | Derivation Path Data      |                   |
| Path[2] | byte (4) | Derivation Path Data      |                   |
| Path[3] | byte (4) | Derivation Path Data      |                   |
| Path[4] | byte (4) | Derivation Path Data      |                   |
| Path[5] | byte (4) | Derivation Path Data      |                   |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #1: Send INVOKE Tx fields

| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x04              |
| P1               | byte (1) | Payload desc                | 0x01              |
| P2               | byte (1) | ignored                     |                   |
| L                | byte (1) | Bytes in payload            | 0x80 (4x32 = 128) |
| SENDER ADDR      | byte (32)| sender address              | (depends)         |
| MAX FEE          | byte (32)| max fee                     | (depends)         |
| CHAIN_ID         | byte (32)| chain_id                    | (depends)         |
| NONCE            | byte (32)| nonce                       | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |


#### Command #2: Number of Calls

| Field            | Type       | Content                     | Expected          |
|------------------|------------|-----------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier      | 0x5A              |
| INS              | byte (1)   | Instruction ID              | 0x04              |
| P1               | byte (1)   | Payload desc                | 0x02              |
| P2               | byte (1)   | ignored                     | 0x00              |
| L                | byte (1)   | Bytes in payload            | 0x20              |
| Num of calls     | bytes (32) | Bytes in payload            | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #3: Call

Each call is sent as a `New Call` APDU (`P2 = 0x00`), followed by as many `Add calldata` APDUs (`P2 = 0x01`) as its calldata needs. The app refuses with `0xff03`:

- an `Add calldata` APDU when no call is in progress, or when the current call already has all its announced calldata;
- a `New Call` APDU while the current call still expects calldata;
- more calldata than announced in `NB CALLDATA`, a felt that is not canonical (`>= P`), or a payload that is not a whole number of felts;
- any other `P2` value.

A call beyond the announced number of calls is refused with `0xff07`.

##### New Call

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x04              |
| P1               | byte (1)   | Payload desc                                   | 0x03              |
| P2               | byte (1)   | New call                                       | 0x00              |
| L                | byte (1)   | Bytes in payload                               | (depends)         |
| TO               | bytes (32) | to                                             | (depends)         |
| SELECTOR         | bytes (32) | selector                                       | (depends)         |
| NB               | bytes (32) | nb_calldata                                    | (depends)         |
| calldata         | bytes (32) | calldata #0                                    | (depends)         |
| calldata         | bytes (32) | calldata #1                                    | (depends)         |
| calldata         | bytes (32) | calldata #2                                    | (depends)         |
| calldata         | bytes (32) | calldata #3                                    | (depends)         |

#### Response 

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

##### Add call data to the current Call

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x04              |
| P1               | byte (1)   | Payload desc                                   | 0x03              |
| P2               | byte (1)   | Add calldata                                   | 0x01              |
| L                | byte (1)   | Bytes in payload                               | (depends)         |
| calldata         | bytes (32) | calldata #0                                    | (depends)         |
| calldata         | bytes (32) | calldata #1                                    | (depends)         |
| calldata         | bytes (32) | calldata #2                                    | (depends)         |
| calldata         | bytes (32) | calldata #3                                    | (depends)         |
| calldata         | bytes (32) | calldata #4                                    | (depends)         |
| calldata         | bytes (32) | calldata #5                                    | (depends)         |
| calldata         | bytes (32) | calldata #6                                    | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |


#### Response (when Tx is complete)

| Field    | Type      | Content           | Note                                  |
|----------|-----------|-------------------|---------------------------------------|
| Tx Hash  | byte (32) | Tx Poseidon Hash  | 32 bytes                              |
| L        | byte (1)  | Sig Length        | 0x41 = 65                             |
| R        | byte (32) | Signature         | (R,S,V) encoded signature             |
| S        | byte (32) | Signature         | (R,S,V) encoded signature             |
| V        | byte (1)  | Signature         | (R,S,V) encoded signature             |
| SW1-SW2  | byte (2)  | Return code       | see list of return codes              |

### Sign DEPLOY_ACCOUNT Tx v1 (see [Starnet Deploy v1](https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v1_deprecated_hash_calculation_3))

This command will return the hash and signature of a Starknet DEPLOY_ACCOUNT Tx version 1

#### Command #0: Set private key

| Field | Type     | Content                     | Expected          |
|-------|----------|-----------------------------|-------------------|
| CLA   | byte (1) | Application Identifier      | 0x5A              |
| INS   | byte (1) | Instruction ID              | 0x06              |
| P1    | byte (1) | Payload desc                | 0x00              |
| P2    | byte (1) | ignored                     |                   |
| L     | byte (1) | Bytes in payload            | (depends)         |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55        |
| Path[1] | byte (4) | Derivation Path Data      |                   |
| Path[2] | byte (4) | Derivation Path Data      |                   |
| Path[3] | byte (4) | Derivation Path Data      |                   |
| Path[4] | byte (4) | Derivation Path Data      |                   |
| Path[5] | byte (4) | Derivation Path Data      |                   |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #1: Send DEPLOY_ACCOUNT Tx fields

All six fields must be canonical felts (`< P`); anything else is refused with `0xff03`. `CONTRACT ADDR` must be the address Starknet derives from `CLASS HASH`, `SALT` and the constructor calldata with the zero deployer: the app recomputes it once the transaction is complete and refuses a mismatch with `0xff05`. The review shows the address, the fees, the class hash, the salt and every constructor argument.

| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x06              |
| P1               | byte (1) | Payload desc                | 0x01              |
| P2               | byte (1) | ignored                     |                   |
| L                | byte (1) | Bytes in payload            |                   |
| CONTRACT ADDR    | byte (32)| contract_address            | (depends)         |
| CLASS HASH       | byte (32)| class_hash                  | (depends)         |
| SALT             | byte (32)| contract_address_salt       | (depends)         |
| MAX FEE          | byte (32)| max_fee                     | (depends)         |
| CHAIN_ID         | byte (32)| chain_id                    | (depends)         |
| NONCE            | byte (32)| nonce                       | (depends)         |


#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #2: Number of constructor calldata

At most 8 constructor calldata are accepted, since every one of them is shown on screen; more is refused with `0xff06`.

| Field            | Type       | Content                     | Expected          |
|------------------|------------|-----------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier      | 0x5A              |
| INS              | byte (1)   | Instruction ID              | 0x06              |
| P1               | byte (1)   | Payload desc                | 0x02              |
| P2               | byte (1)   | ignored                     | 0x00              |
| L                | byte (1)   | Bytes in payload            | 0x20              |
| Num of calldata  | bytes (32) | Bytes in payload            | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #3: Constructor calldata

Chunks of at most 7 felts, until the announced number of calldata is reached. A constructor without arguments is sent as a single empty chunk (`L` = 0). The app refuses with `0xff03` more calldata than announced, a non-canonical felt, or an empty chunk while calldata is still expected.

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x06              |
| P1               | byte (1)   | Payload desc                                   | 0x03              |
| P2               | byte (1)   | ignored                                        |                   |
| L                | byte (1)   | Bytes in payload                               | (depends)         |
| calldata         | bytes (32) | calldata #0                                    | (depends)         |
| calldata         | bytes (32) | calldata #1                                    | (depends)         |
| calldata         | bytes (32) | calldata #2                                    | (depends)         |
| calldata         | bytes (32) | calldata #3                                    | (depends)         |
| calldata         | bytes (32) | calldata #4                                    | (depends)         |
| calldata         | bytes (32) | calldata #5                                    | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Response (when Tx is complete)

| Field    | Type      | Content           | Note                                  |
|----------|-----------|-------------------|---------------------------------------|
| Tx Hash  | byte (32) | Tx Poseidon Hash  | 32 bytes                              |
| L        | byte (1)  | Sig Length        | 0x41 = 65                             |
| R        | byte (32) | Signature         | (R,S,V) encoded signature             |
| S        | byte (32) | Signature         | (R,S,V) encoded signature             |
| V        | byte (1)  | Signature         | (R,S,V) encoded signature             |
| SW1-SW2  | byte (2)  | Return code       | see list of return codes              |