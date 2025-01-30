use crate::{
    context::{
        Call, DeployAccountTransactionV1, DeployAccountTransactionV3, InvokeTransactionV1,
        InvokeTransactionV3, Transaction,
    },
    crypto::{self, HasherTrait},
    types::FieldElement,
};

extern crate alloc;
use alloc::vec::Vec;

const FIELD_ELEMENT_SIZE: usize = 32;

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
            if tx.calls.len() == tx.calls.capacity() {
                let hash_calldata = tx.hasher_calldata.finalize();
                tx.hasher.update(hash_calldata);
                return Some(tx.hasher.finalize());
            }
            None
        }
        Transaction::InvokeV1(tx) => {
            if tx.calls.len() == tx.calls.capacity() {
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
            if tx.constructor_calldata.len() == tx.constructor_calldata.capacity() {
                tx.hasher.update(tx.class_hash);
                tx.hasher.update(tx.contract_address_salt);
                return Some(tx.hasher.finalize());
            }
            None
        }
        Transaction::DeployAccountV1(tx) => {
            if tx.constructor_calldata.len() == tx.constructor_calldata.capacity() {
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

pub fn set_tx_fields(data: &[u8], tx: &mut Transaction) {
    match tx {
        Transaction::InvokeV3(tx) => set_invoke_fields_v3(data, tx),
        Transaction::InvokeV1(tx) => set_invoke_fields_v1(data, tx),
        Transaction::DeployAccountV3(tx) => set_deploy_account_fields_v3(data, tx),
        Transaction::DeployAccountV1(tx) => set_deploy_account_fields_v1(data, tx),
        Transaction::None => panic!("Invalid transaction type"),
    }
}

pub fn set_tx_fees(data: &[u8], tx: &mut Transaction) {
    match tx {
        Transaction::DeployAccountV1(tx) => {
            let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
            tx.max_fee = iter.next().unwrap().into();
        }
        Transaction::DeployAccountV3(tx) => {
            let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
            tx.tip = iter.next().unwrap().into();
            tx.l1_gas_bounds = iter.next().unwrap().into();
            tx.l2_gas_bounds = iter.next().unwrap().into();

            let fee_hash = crate::crypto::poseidon::PoseidonStark252::hash_many(&[
                tx.tip,
                tx.l1_gas_bounds,
                tx.l2_gas_bounds,
            ]);
            tx.hasher.update(fee_hash);
        }
        Transaction::InvokeV1(_) | Transaction::InvokeV3(_) | Transaction::None => {
            panic!("Invalid transaction type")
        }
    }
}

fn set_invoke_fields_v3(data: &[u8], tx: &mut InvokeTransactionV3) {
    tx.version = FieldElement::from(TxVersion::V3 as u8);
    let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
    tx.sender_address = iter.next().unwrap().into();
    tx.tip = iter.next().unwrap().into();
    tx.l1_gas_bounds = iter.next().unwrap().into();
    tx.l2_gas_bounds = iter.next().unwrap().into();
    tx.chain_id = iter.next().unwrap().into();
    tx.nonce = iter.next().unwrap().into();
    tx.data_availability_mode = iter.next().unwrap().into();

    // Update hasher
    tx.hasher.update(FieldElement::INVOKE);
    tx.hasher.update(tx.version);
    tx.hasher.update(tx.sender_address);
    let fee_hash = crate::crypto::poseidon::PoseidonStark252::hash_many(&[
        tx.tip,
        tx.l1_gas_bounds,
        tx.l2_gas_bounds,
    ]);
    tx.hasher.update(fee_hash);
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

fn set_deploy_account_fields_v3(data: &[u8], tx: &mut DeployAccountTransactionV3) {
    tx.version = FieldElement::from(TxVersion::V3 as u8);
    let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
    tx.contract_address = iter.next().unwrap().into();
    tx.chain_id = iter.next().unwrap().into();
    tx.nonce = iter.next().unwrap().into();
    tx.data_availability_mode = iter.next().unwrap().into();
    tx.class_hash = iter.next().unwrap().into();
    tx.contract_address_salt = iter.next().unwrap().into();

    // Update hasher
    tx.hasher.update(FieldElement::DEPLOY_ACCOUNT);
    tx.hasher.update(tx.version);
    tx.hasher.update(tx.contract_address);
}

fn set_deploy_account_fields_v1(data: &[u8], tx: &mut DeployAccountTransactionV1) {
    tx.version = FieldElement::from(TxVersion::V1);
    let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
    tx.contract_address = iter.next().unwrap().into();
    tx.class_hash = iter.next().unwrap().into();
    tx.contract_address_salt = iter.next().unwrap().into();
    tx.chain_id = iter.next().unwrap().into();
    tx.nonce = iter.next().unwrap().into();

    // Update hasher
    tx.hasher.update(FieldElement::DEPLOY_ACCOUNT);
    tx.hasher.update(tx.version);
    tx.hasher.update(tx.contract_address);
    tx.hasher.update(FieldElement::ZERO);
}

#[derive(Debug)]
pub enum SetCallError {
    TooManyCalls = 0xFF01,
}

#[derive(PartialEq)]
pub enum SetCallStep {
    New = 0x00,
    Add = 0x01,
    End = 0x02,
}

impl From<u8> for SetCallStep {
    fn from(value: u8) -> Self {
        match value {
            0x00 => SetCallStep::New,
            0x01 => SetCallStep::Add,
            0x02 => SetCallStep::End,
            _ => panic!("Invalid SetCallStep value"),
        }
    }
}

impl From<SetCallStep> for u8 {
    fn from(value: SetCallStep) -> Self {
        match value {
            SetCallStep::New => 0x00,
            SetCallStep::Add => 0x01,
            SetCallStep::End => 0x02,
        }
    }
}

pub fn set_calldata_nb(tx: &mut Transaction, nb: u8) {
    match tx {
        Transaction::InvokeV3(tx) => {
            tx.calls = Vec::with_capacity(nb as usize);
            tx.hasher_calldata.update(FieldElement::from(nb));
        }
        Transaction::InvokeV1(tx) => {
            tx.calls = Vec::with_capacity(nb as usize);
            tx.hasher_calldata.update(FieldElement::from(nb));
        }
        Transaction::DeployAccountV3(tx) => {
            tx.constructor_calldata = Vec::with_capacity(nb as usize);
        }
        Transaction::DeployAccountV1(tx) => {
            tx.constructor_calldata = Vec::with_capacity(nb as usize);
        }
        Transaction::None => panic!("Invalid transaction type"),
    }
}

pub fn set_calldata(
    data: &[u8],
    p2: SetCallStep,
    tx: &mut Transaction,
) -> Result<(), SetCallError> {
    match tx {
        Transaction::InvokeV3(tx) => {
            set_calldata_invoke(data, p2, &mut tx.calls, &mut tx.hasher_calldata)
        }
        Transaction::InvokeV1(tx) => {
            set_calldata_invoke(data, p2, &mut tx.calls, &mut tx.hasher_calldata)
        }
        Transaction::DeployAccountV3(tx) => {
            let constructor_calldata_hash =
                set_calldata_deploy_account_v3(data, &mut tx.constructor_calldata).unwrap();
            tx.hasher.update(constructor_calldata_hash);
            Ok(())
        }
        Transaction::DeployAccountV1(tx) => {
            let _ = set_calldata_deploy_account_v1(data, &mut tx.constructor_calldata);

            let mut hasher = crypto::pedersen::PedersenHasher::default();
            hasher.update(tx.class_hash);
            hasher.update(tx.contract_address_salt);
            for d in &tx.constructor_calldata {
                hasher.update(*d);
            }
            hasher.update(FieldElement::from(2usize + tx.constructor_calldata.len()));
            tx.hasher.update(hasher.finalize());
            Ok(())
        }
        Transaction::None => panic!("Invalid transaction type"),
    }
}

fn set_calldata_invoke(
    data: &[u8],
    p2: SetCallStep,
    calls: &mut Vec<Call>,
    hasher: &mut impl HasherTrait,
) -> Result<(), SetCallError> {
    match p2 {
        SetCallStep::New => {
            if calls.len() == calls.capacity() {
                return Err(SetCallError::TooManyCalls);
            }
            let mut call = Call::default();
            let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
            call.to = iter.next().unwrap().into();
            call.selector = iter.next().unwrap().into();
            for d in iter {
                call.calldata.push(d.into());
            }
            calls.push(call);
            Ok(())
        }
        SetCallStep::Add | SetCallStep::End => {
            let idx = calls.len() - 1;
            let call: &mut Call = calls.get_mut(idx).unwrap();
            let iter = data.chunks(FIELD_ELEMENT_SIZE);
            for d in iter {
                call.calldata.push(d.into());
            }
            if p2 == SetCallStep::End {
                hasher.update(call.to);
                hasher.update(call.selector);
                hasher.update(FieldElement::from(call.calldata.len() as u8));
                for d in &call.calldata {
                    hasher.update(*d);
                    // Add heartbeat to prevent watchdog reset
                    #[cfg(any(target_os = "nanox", target_os = "stax", target_os = "flex"))]
                    ledger_secure_sdk_sys::seph::heartbeat();
                }
            }
            Ok(())
        }
    }
}

fn set_calldata_deploy_account_v3(
    data: &[u8],
    constructor_calldata: &mut Vec<FieldElement>,
) -> Result<FieldElement, SetCallError> {
    let iter = data.chunks(FIELD_ELEMENT_SIZE);
    for d in iter {
        constructor_calldata.push(d.into());
    }

    let constructor_calldata_hash =
        crypto::poseidon::PoseidonStark252::hash_many(constructor_calldata);

    Ok(constructor_calldata_hash)
}

fn set_calldata_deploy_account_v1(
    data: &[u8],
    constructor_calldata: &mut Vec<FieldElement>,
) -> Result<(), SetCallError> {
    let iter = data.chunks(FIELD_ELEMENT_SIZE);
    for d in iter {
        constructor_calldata.push(d.into());
    }
    Ok(())
}
