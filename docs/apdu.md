# APDU protocol description

This document aims to provide a description of the APDU protocol supported by the app, explaining what each instruction does, the expected parameters and return values

## General Structure

The general structure of a reqeuest and response is as followed:

### Request / Command

| Field   | Type     | Content                | Note                   |
|:--------|:---------|:-----------------------|------------------------|
| CLA     | byte (1) | Application Identifier | 0x80, To be determined |
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
| 0x6400      | Execution Error         |
| 0x6982      | Empty buffer            |
| 0x6983      | Output buffer too small |
| 0x6986      | Command not allowed     |
| 0x6D00      | Unknown                 |
| 0x6E00      | CLA not supported       |
| 0xE000      | Panic                   |
| 0x9000      | Success                 |

---

## Commands definitions

### GetVersion

This command will return the app version

#### Command

| Field | Type     | Content                | Expected |
|-------|----------|------------------------|----------|
| CLA   | byte (1) | Application Identifier | 0x80     |
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
| CLA     | byte (1) | Application Identifier    | 0x80            |
| INS     | byte (1) | Instruction ID            | 0x01            |
| P1      | byte (1) | Parameter 1               | ignored         |
| P2      | byte (1) | Parameter 1               | ignored         |
| L       | byte (1) | Bytes in payload          | (depends)       |
| PathN   | byte (1) | Number of path components | 6               |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55      |
| Path[1] | byte (4) | Derivation Path Data      | ?               |
| Path[2] | byte (4) | Derivation Path Data      | ?               |
| Path[3] | byte (4) | Derivation Path Data      | ?               |
| Path[4] | byte (4) | Derivation Path Data      | ?               |
| Path[5] | byte (4) | Derivation Path Data      | ?               |

#### Response

| Field      | Type      | Content           | Note                     |
| ---------- | --------- | ----------------- | ------------------------ |
| PK_LEN     | byte (1)  | Bytes in PKEY     |                          |
| PKEY       | byte (??) | Public key bytes  |                          |
| SW1-SW2    | byte (2)  | Return code       | see list of return codes |

### Sign

This command will return a signature of the passed payload

#### Command #1

| Field | Type     | Content                     | Expected          |
|-------|----------|-----------------------------|-------------------|
| CLA   | byte (1) | Application Identifier      | 0x80              |
| INS   | byte (1) | Instruction ID              | 0x02              |
| P1    | byte (1) | Payload desc                | 0                 |
| P2    | byte (1) | ignored                     |                   |
| L     | byte (1) | Bytes in payload            | (depends)         |
| PathN   | byte (1) | Number of path components | 6                 |
| Path[0] | byte (4) | Derivation Path Data      | 0x80000A55        |
| Path[1] | byte (4) | Derivation Path Data      | ?                 |
| Path[2] | byte (4) | Derivation Path Data      | ?                 |
| Path[3] | byte (4) | Derivation Path Data      | ?                 |
| Path[4] | byte (4) | Derivation Path Data      | ?                 |
| Path[5] | byte (4) | Derivation Path Data      | ?                 |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| SIG      | byte (64) | Signature   | (R,S) encoded signature               |
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |

#### Command #2

| Field | Type     | Content                     | Expected          |
|-------|----------|-----------------------------|-------------------|
| CLA   | byte (1) | Application Identifier      | 0x80              |
| INS   | byte (1) | Instruction ID              | 0x02              |
| P1    | byte (1) | Payload desc                | 1                 |
| P2    | byte (1) | ignored                     | display ?         |
| L     | byte (1) | Bytes in payload            | 20                |
| Hash  | byte (32)| Pedersen hash               | (depends)         |

#### Response

| Field    | Type      | Content     | Note                                  |
|----------|-----------|-------------|---------------------------------------|
| L        | byte (1)  | Sig Length  | 0x41 = 65                             |
| SIG      | byte (65) | Signature   | (R,S, V) encoded signature            |
| SW1-SW2  | byte (2)  | Return code | see list of return codes              |
