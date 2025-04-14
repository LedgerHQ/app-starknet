# APDU protocol description

This document aims to provide a description of the APDU protocol supported by the app, explaining what each instruction does, the expected parameters and return values

## General Structure

The general structure of a reqeuest and response is as followed:

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
| ANSWER  | byte (?) | Answer      | depends on the command   |
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

This command returns the public key corresponding to the secret key found at the given [EIP-2645](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-2645.md) path 

#### Command

| Field   | Type     | Content                   | Expected        |
|---------|----------|---------------------------|-----------------|
| CLA     | byte (1) | Application Identifier    | 0x5A            |
| INS     | byte (1) | Instruction ID            | 0x01            |
| P1      | byte (1) | Parameter 1               | if not 0, user will have to confirm          |
| P2      | byte (1) | Parameter 2               | ignored         |
| L       | byte (1) | Bytes in payload          | 0x18            |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55      |
| Path[1] | byte (4) | Derivation Path Data      | ?               |
| Path[2] | byte (4) | Derivation Path Data      | ?               |
| Path[3] | byte (4) | Derivation Path Data      | ?               |
| Path[4] | byte (4) | Derivation Path Data      | ?               |
| Path[5] | byte (4) | Derivation Path Data      | ?               |

#### Response

| Field      | Type      | Content           | Note                     |
| ---------- | --------- | ----------------- | ------------------------ |
| PK_LEN     | byte (1)  | Bytes in PKEY     | 64                       |
| PKEY       | byte (??) | Public key bytes  | 32 (x) + 32 (y)          |
| SW1-SW2    | byte (2)  | Return code       | see list of return codes |

### Sign Hash

This command will return the signature of a Pedersen or Poseidon Hash

#### Command #0: Set private key

| Field | Type     | Content                     | Expected          |
|-------|----------|-----------------------------|-------------------|
| CLA   | byte (1) | Application Identifier      | 0x5A              |
| INS   | byte (1) | Instruction ID              | 0x02              |
| P1    | byte (1) | Payload desc                | 0x00              |
| P2    | byte (1) | ignored                     |                   |
| L     | byte (1) | Bytes in payload            | (depends)         |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55        |
| Path[1] | byte (4) | Derivation Path Data      | ?                 |
| Path[2] | byte (4) | Derivation Path Data      | ?                 |
| Path[3] | byte (4) | Derivation Path Data      | ?                 |
| Path[4] | byte (4) | Derivation Path Data      | ?                 |
| Path[5] | byte (4) | Derivation Path Data      | ?                 |

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
| L     | byte (1)   | Bytes in payload            | 0x20              |
| Hash  | bytes (32) | Hash bytes                  | (depends)         |

#### Response

| Field    | Type      | Content           | Note                                  |
|----------|-----------|-------------------|---------------------------------------|
| L        | byte (1)  | Sig Length        | 0x41 = 65                             |
| R        | byte (32) | Signature         | (R,S,V) encoded signature             |
| S        | byte (32) | Signature         | (R,S,V) encoded signature             |
| V        | byte (1)  | Signature         | (R,S,V) encoded signature             |
| SW1-SW2  | byte (2)  | Return code       | see list of return codes              |


### Sign Invoke Tx (see [Starnet Tx v3](https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v3_hash_calculation))

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
| Path[1] | byte (4) | Derivation Path Data      | ?                 |
| Path[2] | byte (4) | Derivation Path Data      | ?                 |
| Path[3] | byte (4) | Derivation Path Data      | ?                 |
| Path[4] | byte (4) | Derivation Path Data      | ?                 |
| Path[5] | byte (4) | Derivation Path Data      | ?                 |

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
| L                | byte (1) | Bytes in payload            | 0xE0 (7x32 = 224) |
| Account Address  | byte (32)| sender address              | (depends)         |
| ChainID          | byte (32)| chain_id                    | (depends)         |
| Nonce            | byte (32)| nonce                       | (depends)         |
| DA mode          | byte (32)| data_availability_mode      | (depends)         |

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
| L                | byte (1) | Bytes in payload            |                   |
| Tip              | byte (32)| tip                         | (depends)         |
| Fee L1           | byte (32)| l1_gas_bounds               | (depends)         |
| Fee L2           | byte (32)| l2_gas_bounds               | (depends)         |
| Data L1          | byte (32)| l1_data_gas_bounds          | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #3: Send Paymaster data

| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x03              |
| P1               | byte (1) | Payload desc                | 0x02              |
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

#### Command #4: Number of Calls

| Field            | Type       | Content                     | Expected          |
|------------------|------------|-----------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier      | 0x5A              |
| INS              | byte (1)   | Instruction ID              | 0x03              |
| P1               | byte (1)   | Payload desc                | 0x04              |
| P2               | byte (1)   | ignored                     | 0x00              |
| L                | byte (1)   | Bytes in payload            | 0x20              |
| Num of calls     | bytes (32) | Bytes in payload            | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #5: Call

##### New

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x03              |
| P1               | byte (1)   | Payload desc                                   | 0x05              |
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

##### Add

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x03              |
| P1               | byte (1)   | Payload desc                                   | 0x05              |
| P2               | byte (1)   | Next calldata                                  | 0x01              |
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

##### End

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x03              |
| P1               | byte (1)   | Payload desc                                   | 0x05              |
| P2               | byte (1)   | Next calldata                                  | 0x02              |
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

### Sign Deploy Account Tx (see [Starnet Deploy v3](https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v3_hash_calculation_3))

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
| Path[1] | byte (4) | Derivation Path Data      | ?                 |
| Path[2] | byte (4) | Derivation Path Data      | ?                 |
| Path[3] | byte (4) | Derivation Path Data      | ?                 |
| Path[4] | byte (4) | Derivation Path Data      | ?                 |
| Path[5] | byte (4) | Derivation Path Data      | ?                 |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #1: Send DEPLOY_ACCOUNT Tx fields


| Field            | Type     | Content                     | Expected          |
|------------------|----------|-----------------------------|-------------------|
| CLA              | byte (1) | Application Identifier      | 0x5A              |
| INS              | byte (1) | Instruction ID              | 0x05              |
| P1               | byte (1) | Payload desc                | 0x01              |
| P2               | byte (1) | ignored                     |                   |
| L                | byte (1) | Bytes in payload            |                   |
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
| L                | byte (1) | Bytes in payload            |                   |
| Tip              | byte (32)| tip                         | (depends)         |
| Fee L1           | byte (32)| l1_gas_bounds               | (depends)         |
| Fee L2           | byte (32)| l2_gas_bounds               | (depends)         |
| Data L1          | byte (32)| l1_data_gas_bounds          | (depends)         |

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

| Field            | Type       | Content                     | Expected          |
|------------------|------------|-----------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier      | 0x5A              |
| INS              | byte (1)   | Instruction ID              | 0x03              |
| P1               | byte (1)   | Payload desc                | 0x04              |
| P2               | byte (1)   | ignored                     | 0x00              |
| L                | byte (1)   | Bytes in payload            | 0x20              |
| Num of calldata  | bytes (32) | Bytes in payload            | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #5: Constructor calldata

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x03              |
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


### Sign TxV1 (see [Starnet Tx v1](https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v1_deprecated_hash_calculation))

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
| Path[1] | byte (4) | Derivation Path Data      | ?                 |
| Path[2] | byte (4) | Derivation Path Data      | ?                 |
| Path[3] | byte (4) | Derivation Path Data      | ?                 |
| Path[4] | byte (4) | Derivation Path Data      | ?                 |
| Path[5] | byte (4) | Derivation Path Data      | ?                 |

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
| L                | byte (1) | Bytes in payload            | 0xE0 (7x32 = 224) |
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

##### New

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

##### Add

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x04              |
| P1               | byte (1)   | Payload desc                                   | 0x03              |
| P2               | byte (1)   | Next calldata                                  | 0x01              |
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

##### End

| Field            | Type       | Content                                        | Expected          |
|------------------|------------|------------------------------------------------|-------------------|
| CLA              | byte (1)   | Application Identifier                         | 0x5A              |
| INS              | byte (1)   | Instruction ID                                 | 0x04              |
| P1               | byte (1)   | Payload desc                                   | 0x03              |
| P2               | byte (1)   | Next calldata                                  | 0x02              |
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

### Sign Deploy Account Tx v1 (see [Starnet Deploy v1](https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v1_deprecated_hash_calculation_3))

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
| Path[1] | byte (4) | Derivation Path Data      | ?                 |
| Path[2] | byte (4) | Derivation Path Data      | ?                 |
| Path[3] | byte (4) | Derivation Path Data      | ?                 |
| Path[4] | byte (4) | Derivation Path Data      | ?                 |
| Path[5] | byte (4) | Derivation Path Data      | ?                 |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #1: Send DEPLOY_ACCOUNT Tx fields

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

#### Command #4: Number of constructor calldata

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

#### Command #5: Constructor calldata

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