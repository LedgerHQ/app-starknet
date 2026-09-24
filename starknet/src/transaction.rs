use crate::{
    context::{
        Call, DeployAccountTransactionV1, DeployAccountTransactionV3, InvokeTransactionV1,
        InvokeTransactionV3, Transaction,
    },
    crypto::{self, HasherTrait},
    types::{FieldElement, P},
};

extern crate alloc;
use alloc::vec::Vec;

const FIELD_ELEMENT_SIZE: usize = 32;

/// Constructor arguments are all shown on screen, so their number is capped.
/// Common account constructors take one to three.
pub const MAX_CONSTRUCTOR_CALLDATA: usize = 8;

/// Why the app refuses an APDU while it parses a transaction. Each of these
/// drops the signing request: the host has to start again from P1=0.
#[derive(Clone, Copy, Debug)]
pub enum ParseError {
    /// The APDU is out of sequence, or its payload contradicts what the
    /// transaction announced so far.
    BadState = 0xFF03,
    /// The fee APDU does not hold a tip and resource bounds in their wire format.
    InvalidFees = 0xFF04,
    TooManyConstructorArgs = 0xFF06,
    /// A call beyond the announced number of calls.
    TooManyCalls = 0xFF07,
}

/// Splits an APDU payload into felts, refusing a partial or non-canonical one.
/// A felt >= P hashes like its reduced value but reads differently wherever
/// the app interprets it, so the review could describe another transaction
/// than the one signed.
fn parse_felts(data: &[u8]) -> Result<Vec<FieldElement>, ParseError> {
    if !data.len().is_multiple_of(FIELD_ELEMENT_SIZE) {
        return Err(ParseError::BadState);
    }
    let felts: Vec<FieldElement> = data.chunks(FIELD_ELEMENT_SIZE).map(|d| d.into()).collect();
    if felts.iter().any(|f| *f >= P) {
        return Err(ParseError::BadState);
    }
    Ok(felts)
}

/// Reads a count carried in a felt. The whole felt is hashed, so it has to be
/// read back whole: a value past u32 would be truncated, and the app would
/// parse a different transaction from the one signed.
fn felt_to_count(fe: FieldElement) -> Result<usize, ParseError> {
    if fe.value[..28].iter().any(|&b| b != 0) {
        return Err(ParseError::BadState);
    }
    Ok(u32::from_be_bytes(fe.value[28..].try_into().unwrap()) as usize)
}

pub enum TxVersion {
    V1 = 1,
    V3 = 3,
}

impl From<TxVersion> for FieldElement {
    fn from(version: TxVersion) -> Self {
        FieldElement::from(version as u8)
    }
}

impl From<FieldElement> for TxVersion {
    fn from(version: FieldElement) -> Self {
        match version.into() {
            1u8 => TxVersion::V1,
            3u8 => TxVersion::V3,
            _ => panic!("Invalid transaction version"),
        }
    }
}

pub fn tx_complete(tx: &mut Transaction) -> Option<FieldElement> {
    match tx {
        Transaction::InvokeV3(tx) => {
            if (tx.nb_rcv_calls == tx.nb_calls) && (tx.call.nb_rcv_calldata == tx.call.nb_calldata)
            {
                let hash_calldata = tx.hasher_calldata.finalize();
                tx.hasher.update(hash_calldata);
                return Some(tx.hasher.finalize());
            }
            None
        }
        Transaction::InvokeV1(tx) => {
            if (tx.nb_rcv_calls == tx.nb_calls) && (tx.call.nb_rcv_calldata == tx.call.nb_calldata)
            {
                tx.hasher_calldata
                    .update(FieldElement::from(tx.hasher_calldata.get_nb_fe() as u8));
                let hash_calldata = tx.hasher_calldata.finalize();
                tx.hasher.update(hash_calldata);
                tx.hasher.update(tx.max_fee);
                tx.hasher.update(tx.chain_id);
                tx.hasher.update(tx.nonce);
                tx.hasher.update(FieldElement::from(8u8));
                return Some(tx.hasher.finalize());
            }
            None
        }
        Transaction::DeployAccountV3(tx) => {
            if tx.constructor_calldata.len() == tx.nb_constructor_calldata {
                tx.hasher.update(tx.hasher_calldata.finalize());
                tx.hasher.update(tx.class_hash);
                tx.hasher.update(tx.contract_address_salt);
                return Some(tx.hasher.finalize());
            }
            None
        }
        Transaction::DeployAccountV1(tx) => {
            if tx.constructor_calldata.len() == tx.nb_constructor_calldata {
                let mut hasher = crypto::pedersen::PedersenHasher::default();
                hasher.update(tx.class_hash);
                hasher.update(tx.contract_address_salt);
                for d in &tx.constructor_calldata {
                    hasher.update(*d);
                }
                hasher.update(FieldElement::from(2usize + tx.constructor_calldata.len()));
                tx.hasher.update(hasher.finalize());
                tx.hasher.update(tx.max_fee);
                tx.hasher.update(tx.chain_id);
                tx.hasher.update(tx.nonce);
                tx.hasher.update(FieldElement::from(8u8));
                return Some(tx.hasher.finalize());
            }
            None
        }
        Transaction::None => None,
    }
}

pub fn set_tx_fields(data: &[u8], tx: &mut Transaction) -> Result<(), ParseError> {
    match tx {
        Transaction::InvokeV3(tx) => set_invoke_fields_v3(data, tx),
        Transaction::InvokeV1(tx) => set_invoke_fields_v1(data, tx),
        Transaction::DeployAccountV3(tx) => return set_deploy_account_fields_v3(data, tx),
        Transaction::DeployAccountV1(tx) => return set_deploy_account_fields_v1(data, tx),
        Transaction::None => panic!("Invalid transaction type"),
    }
    Ok(())
}

/// The six Deploy Account fields. They feed the on-device address check and
/// the review, so they are held to the same rule as calldata: whole, canonical
/// felts, exactly as many as expected.
fn parse_deploy_account_fields(data: &[u8]) -> Result<[FieldElement; 6], ParseError> {
    parse_felts(data)?
        .try_into()
        .map_err(|_| ParseError::BadState)
}

/// V3 tip and resource bounds, as signed.
pub struct Fees {
    pub tip: FieldElement,
    pub l1_gas_bounds: FieldElement,
    pub l2_gas_bounds: FieldElement,
    pub l1_data_gas_bounds: FieldElement,
    pub hash: FieldElement,
}

/// Parses the V3 fee APDU: tip, then the L1 gas, L2 gas and optional L1 data
/// gas bounds. Each bound packs, big-endian, the resource name right-aligned
/// in 8 bytes, max_amount (u64) and max_price_per_unit (u128). The review
/// decodes amount and price from those bytes while Poseidon absorbs the word
/// modulo P, so any word off that layout could hash as one bound and display
/// as another. Checking the name also keeps the word below 2^248, hence
/// canonical.
fn parse_fees(data: &[u8]) -> Result<Fees, ParseError> {
    const TAGS: [&[u8; 8]; 3] = [b"\0\0L1_GAS", b"\0\0L2_GAS", b"\0L1_DATA"];

    if data.len() != 3 * FIELD_ELEMENT_SIZE && data.len() != 4 * FIELD_ELEMENT_SIZE {
        return Err(ParseError::InvalidFees);
    }
    let words: Vec<FieldElement> = data.chunks(FIELD_ELEMENT_SIZE).map(|d| d.into()).collect();

    // The tip is a u64 on the wire.
    if words[0].value[..24].iter().any(|&b| b != 0) {
        return Err(ParseError::InvalidFees);
    }
    for (word, tag) in words[1..].iter().zip(TAGS) {
        if &word.value[..8] != tag {
            return Err(ParseError::InvalidFees);
        }
    }

    let mut hasher = crypto::poseidon::PoseidonHasher::default();
    for w in &words {
        hasher.update(*w);
    }
    Ok(Fees {
        tip: words[0],
        l1_gas_bounds: words[1],
        l2_gas_bounds: words[2],
        l1_data_gas_bounds: words.get(3).copied().unwrap_or(FieldElement::ZERO),
        hash: hasher.finalize(),
    })
}

pub fn set_tx_fees(data: &[u8], tx: &mut Transaction) -> Result<(), ParseError> {
    let fees = parse_fees(data)?;
    match tx {
        Transaction::DeployAccountV3(tx) => {
            tx.tip = fees.tip;
            tx.l1_gas_bounds = fees.l1_gas_bounds;
            tx.l2_gas_bounds = fees.l2_gas_bounds;
            tx.l1_data_gas_bounds = fees.l1_data_gas_bounds;
            tx.hasher.update(fees.hash);
        }
        Transaction::InvokeV3(tx) => {
            tx.tip = fees.tip;
            tx.l1_gas_bounds = fees.l1_gas_bounds;
            tx.l2_gas_bounds = fees.l2_gas_bounds;
            tx.l1_data_gas_bounds = fees.l1_data_gas_bounds;
            tx.hasher.update(fees.hash);
        }
        Transaction::InvokeV1(_) | Transaction::DeployAccountV1(_) | Transaction::None => {
            panic!("Invalid transaction type")
        }
    }
    Ok(())
}

fn set_invoke_fields_v3(data: &[u8], tx: &mut InvokeTransactionV3) {
    tx.version = FieldElement::from(TxVersion::V3 as u8);
    let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
    tx.sender_address = iter.next().unwrap().into();
    tx.chain_id = iter.next().unwrap().into();
    tx.nonce = iter.next().unwrap().into();
    tx.data_availability_mode = iter.next().unwrap().into();

    // Update hasher
    tx.hasher.update(FieldElement::INVOKE);
    tx.hasher.update(tx.version);
    tx.hasher.update(tx.sender_address);
}

fn set_invoke_fields_v1(data: &[u8], tx: &mut InvokeTransactionV1) {
    tx.version = FieldElement::from(TxVersion::V1 as u8);
    let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
    tx.sender_address = iter.next().unwrap().into();
    tx.max_fee = iter.next().unwrap().into();
    tx.chain_id = iter.next().unwrap().into();
    tx.nonce = iter.next().unwrap().into();

    // Update hasher
    tx.hasher.update(FieldElement::INVOKE);
    tx.hasher.update(tx.version);
    tx.hasher.update(tx.sender_address);
    tx.hasher.update(FieldElement::ZERO);
}

// For future use: currently paymaster_data is always empty.
// See https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v3_transaction_fields
pub fn set_paymaster_data(_data: &[u8], _p2: u8, tx: &mut Transaction) {
    match tx {
        Transaction::InvokeV3(tx) => {
            let paymaster_hash =
                crate::crypto::poseidon::PoseidonStark252::hash_many(&tx.paymaster_data);
            tx.hasher.update(paymaster_hash);
            tx.hasher.update(tx.chain_id);
            tx.hasher.update(tx.nonce);
            tx.hasher.update(tx.data_availability_mode);
        }

        Transaction::DeployAccountV3(tx) => {
            let paymaster_hash =
                crate::crypto::poseidon::PoseidonStark252::hash_many(&tx.paymaster_data);
            tx.hasher.update(paymaster_hash);
            tx.hasher.update(tx.chain_id);
            tx.hasher.update(tx.nonce);
            tx.hasher.update(tx.data_availability_mode);
        }
        _ => panic!("Invalid transaction type"),
    }
}

// For future use: currently account_deployment_data is always empty.
// See https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v3_transaction_fields
pub fn set_account_deployment_data(_data: &[u8], _p2: u8, tx: &mut Transaction) {
    match tx {
        Transaction::InvokeV3(tx) => {
            let account_deployment_hash =
                crate::crypto::poseidon::PoseidonStark252::hash_many(&tx.account_deployment_data);
            tx.hasher.update(account_deployment_hash);
        }
        _ => panic!("Invalid transaction type"),
    }
}

fn set_deploy_account_fields_v3(
    data: &[u8],
    tx: &mut DeployAccountTransactionV3,
) -> Result<(), ParseError> {
    let [contract_address, chain_id, nonce, data_availability_mode, class_hash, salt] =
        parse_deploy_account_fields(data)?;
    tx.version = FieldElement::from(TxVersion::V3 as u8);
    tx.contract_address = contract_address;
    tx.chain_id = chain_id;
    tx.nonce = nonce;
    tx.data_availability_mode = data_availability_mode;
    tx.class_hash = class_hash;
    tx.contract_address_salt = salt;

    // Update hasher
    tx.hasher.update(FieldElement::DEPLOY_ACCOUNT);
    tx.hasher.update(tx.version);
    tx.hasher.update(tx.contract_address);
    Ok(())
}

fn set_deploy_account_fields_v1(
    data: &[u8],
    tx: &mut DeployAccountTransactionV1,
) -> Result<(), ParseError> {
    let [contract_address, class_hash, salt, max_fee, chain_id, nonce] =
        parse_deploy_account_fields(data)?;
    tx.version = FieldElement::from(TxVersion::V1);
    tx.contract_address = contract_address;
    tx.class_hash = class_hash;
    tx.contract_address_salt = salt;
    tx.max_fee = max_fee;
    tx.chain_id = chain_id;
    tx.nonce = nonce;

    // Update hasher
    tx.hasher.update(FieldElement::DEPLOY_ACCOUNT);
    tx.hasher.update(tx.version);
    tx.hasher.update(tx.contract_address);
    tx.hasher.update(FieldElement::ZERO);
    Ok(())
}

#[derive(PartialEq)]
pub enum SetCallStep {
    New = 0x00,
    Add = 0x01,
}

impl TryFrom<u8> for SetCallStep {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(SetCallStep::New),
            0x01 => Ok(SetCallStep::Add),
            _ => Err(ParseError::BadState),
        }
    }
}

impl From<SetCallStep> for u8 {
    fn from(value: SetCallStep) -> Self {
        match value {
            SetCallStep::New => 0x00,
            SetCallStep::Add => 0x01,
        }
    }
}

pub fn set_calldata_nb(data: &[u8], tx: &mut Transaction) -> Result<(), ParseError> {
    let felts = parse_felts(data)?;
    let [nb] = felts[..] else {
        return Err(ParseError::BadState);
    };
    let count = felt_to_count(nb)?;
    match tx {
        Transaction::InvokeV3(tx) => {
            tx.nb_calls = count;
            tx.hasher_calldata.update(nb);
        }
        Transaction::InvokeV1(tx) => {
            tx.nb_calls = count;
            tx.hasher_calldata.update(nb);
        }
        Transaction::DeployAccountV3(DeployAccountTransactionV3 {
            nb_constructor_calldata,
            constructor_calldata,
            ..
        })
        | Transaction::DeployAccountV1(DeployAccountTransactionV1 {
            nb_constructor_calldata,
            constructor_calldata,
            ..
        }) => {
            if count > MAX_CONSTRUCTOR_CALLDATA {
                return Err(ParseError::TooManyConstructorArgs);
            }
            *nb_constructor_calldata = count;
            *constructor_calldata = Vec::with_capacity(count);
        }
        Transaction::None => panic!("Invalid transaction type"),
    }
    Ok(())
}

pub fn set_calldata(data: &[u8], p2: u8, tx: &mut Transaction) -> Result<(), ParseError> {
    match tx {
        Transaction::InvokeV3(tx) => set_calldata_invoke(
            data,
            p2.try_into()?,
            &mut tx.call,
            &mut tx.hasher_calldata,
            tx.nb_calls,
            &mut tx.nb_rcv_calls,
        ),
        Transaction::InvokeV1(tx) => set_calldata_invoke(
            data,
            p2.try_into()?,
            &mut tx.call,
            &mut tx.hasher_calldata,
            tx.nb_calls,
            &mut tx.nb_rcv_calls,
        ),
        Transaction::DeployAccountV3(tx) => {
            let felts = add_constructor_calldata(
                data,
                tx.nb_constructor_calldata,
                &mut tx.constructor_calldata,
            )?;
            for d in felts {
                tx.hasher_calldata.update(d);
            }
            Ok(())
        }
        Transaction::DeployAccountV1(tx) => add_constructor_calldata(
            data,
            tx.nb_constructor_calldata,
            &mut tx.constructor_calldata,
        )
        .map(|_| ()),
        Transaction::None => panic!("Invalid transaction type"),
    }
}

/// Streams one Invoke call, or a continuation of it, into the calldata hash.
/// The review is built from `call`, so every felt hashed has to be accounted
/// for there: a call's calldata is complete before the next call starts,
/// continuation chunks only extend a call already started, and neither calls
/// nor calldata go past what was announced. Everything is checked before the
/// hasher sees a single felt.
fn set_calldata_invoke(
    data: &[u8],
    p2: SetCallStep,
    call: &mut Call,
    hasher: &mut impl HasherTrait,
    nb_calls: usize,
    nb_rcv_calls: &mut usize,
) -> Result<(), ParseError> {
    let felts = parse_felts(data)?;
    let call_open = call.nb_rcv_calldata < call.nb_calldata;
    match p2 {
        SetCallStep::New => {
            if call_open || felts.len() < 3 {
                return Err(ParseError::BadState);
            }
            if *nb_rcv_calls >= nb_calls {
                return Err(ParseError::TooManyCalls);
            }
            let nb_calldata = felt_to_count(felts[2])?;
            if felts.len() - 3 > nb_calldata {
                return Err(ParseError::BadState);
            }
            *call = Call {
                to: felts[0],
                selector: felts[1],
                nb_calldata,
                nb_rcv_calldata: felts.len() - 3,
                calldata: felts[3..].to_vec(),
            };
            *nb_rcv_calls += 1;
        }
        SetCallStep::Add => {
            if !call_open
                || felts.is_empty()
                || felts.len() > call.nb_calldata - call.nb_rcv_calldata
            {
                return Err(ParseError::BadState);
            }
            call.nb_rcv_calldata += felts.len();
        }
    }
    for d in felts {
        hasher.update(d);
    }
    Ok(())
}

/// Appends a chunk of constructor calldata, never past the announced length.
/// A constructor without arguments is sent as a single empty chunk; otherwise
/// every chunk carries calldata.
fn add_constructor_calldata(
    data: &[u8],
    nb_constructor_calldata: usize,
    constructor_calldata: &mut Vec<FieldElement>,
) -> Result<Vec<FieldElement>, ParseError> {
    let felts = parse_felts(data)?;
    if (felts.is_empty() && nb_constructor_calldata != 0)
        || constructor_calldata.len() + felts.len() > nb_constructor_calldata
    {
        return Err(ParseError::BadState);
    }
    constructor_calldata.extend_from_slice(&felts);
    Ok(felts)
}

/// The address Starknet deploys an account to, from the zero deployer:
/// compute_hash_on_elements(["STARKNET_CONTRACT_ADDRESS", 0, salt, class_hash,
/// compute_hash_on_elements(constructor_calldata)]) mod 2^251 - 256.
pub fn contract_address(
    class_hash: FieldElement,
    salt: FieldElement,
    constructor_calldata: &[FieldElement],
) -> FieldElement {
    // 2^251 - 256
    const ADDR_BOUND: FieldElement = {
        let mut value = [0xffu8; 32];
        value[0] = 0x07;
        value[31] = 0x00;
        FieldElement { value }
    };

    let mut calldata_hasher = crypto::pedersen::PedersenHasher::default();
    for d in constructor_calldata {
        calldata_hasher.update(*d);
    }
    calldata_hasher.update(FieldElement::from(constructor_calldata.len()));

    let mut hasher = crypto::pedersen::PedersenHasher::default();
    hasher.update(FieldElement::from(&b"STARKNET_CONTRACT_ADDRESS"[..]));
    hasher.update(FieldElement::ZERO);
    hasher.update(salt);
    hasher.update(class_hash);
    hasher.update(calldata_hasher.finalize());
    hasher.update(FieldElement::from(5usize));

    let address = hasher.finalize();
    match address >= ADDR_BOUND {
        true => address - ADDR_BOUND,
        false => address,
    }
}
