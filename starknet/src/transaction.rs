use crate::{
    context::{Call, DeployAccountTransaction, InvokeTransaction, Transaction},
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

pub fn tx_complete(tx: &Transaction) -> bool {
    match tx {
        Transaction::Invoke(tx) => tx.calls.len() == tx.calls.capacity(),
        Transaction::DeployAccount(tx) => {
            tx.constructor_calldata.len() == tx.constructor_calldata.capacity()
        }
        Transaction::None => true,
    }
}

pub fn set_tx_fields(data: &[u8], tx: &mut Transaction, version: TxVersion) {
    match tx {
        Transaction::Invoke(tx) => match version {
            TxVersion::V1 => set_invoke_fields_v1(data, tx),
            TxVersion::V3 => set_invoke_fields_v3(data, tx),
        },
        Transaction::DeployAccount(tx) => match version {
            TxVersion::V1 => set_deploy_account_fields_v1(data, tx),
            TxVersion::V3 => set_deploy_account_fields_v3(data, tx),
        },
        Transaction::None => panic!("Invalid transaction type"),
    }
}

pub fn set_tx_fees(data: &[u8], tx: &mut Transaction) {
    match tx {
        Transaction::DeployAccount(tx) => match tx.version.into() {
            TxVersion::V1 => {
                let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
                tx.max_fee = iter.next().unwrap().into();
            }
            TxVersion::V3 => {
                let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
                tx.tip = iter.next().unwrap().into();
                tx.l1_gas_bounds = iter.next().unwrap().into();
                tx.l2_gas_bounds = iter.next().unwrap().into();
            }
        },
        Transaction::Invoke(_) | Transaction::None => panic!("Invalid transaction type"),
    }
}

fn set_invoke_fields_v3(data: &[u8], tx: &mut InvokeTransaction) {
    tx.version = FieldElement::from(3u8);
    let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
    tx.sender_address = iter.next().unwrap().into();
    tx.tip = iter.next().unwrap().into();
    tx.l1_gas_bounds = iter.next().unwrap().into();
    tx.l2_gas_bounds = iter.next().unwrap().into();
    tx.chain_id = iter.next().unwrap().into();
    tx.nonce = iter.next().unwrap().into();
    tx.data_availability_mode = iter.next().unwrap().into();
}

fn set_invoke_fields_v1(data: &[u8], tx: &mut InvokeTransaction) {
    tx.version = FieldElement::from(1u8);
    let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
    tx.sender_address = iter.next().unwrap().into();
    tx.max_fee = iter.next().unwrap().into();
    tx.chain_id = iter.next().unwrap().into();
    tx.nonce = iter.next().unwrap().into();
}

// For future use: currently paymaster_data is always empty.
// See https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v3_transaction_fields
pub fn set_paymaster_data(_data: &[u8], _p2: u8, _tx: &mut Transaction) {}
//pub fn set_paymaster_data(_data: &[u8], _p2: u8, _paymaster_data: &mut [FieldElement]) {}

// For future use: currently account_deployment_data is always empty.
// See https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v3_transaction_fields
pub fn set_account_deployment_data(_data: &[u8], _p2: u8, _tx: &mut Transaction) {}
// pub fn set_account_deployment_data(
//     _data: &[u8],
//     _p2: u8,
//     _account_deployment_data: &mut [FieldElement],
// ) {
// }

fn set_deploy_account_fields_v3(data: &[u8], tx: &mut DeployAccountTransaction) {
    tx.version = FieldElement::from(TxVersion::V3);
    let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
    tx.contract_address = iter.next().unwrap().into();
    tx.chain_id = iter.next().unwrap().into();
    tx.nonce = iter.next().unwrap().into();
    tx.data_availability_mode = iter.next().unwrap().into();
    tx.class_hash = iter.next().unwrap().into();
    tx.contract_address_salt = iter.next().unwrap().into();
}

fn set_deploy_account_fields_v1(data: &[u8], tx: &mut DeployAccountTransaction) {
    tx.version = FieldElement::from(TxVersion::V1);
    let mut iter = data.chunks(FIELD_ELEMENT_SIZE);
    tx.contract_address = iter.next().unwrap().into();
    tx.class_hash = iter.next().unwrap().into();
    tx.contract_address_salt = iter.next().unwrap().into();
    tx.chain_id = iter.next().unwrap().into();
    tx.nonce = iter.next().unwrap().into();
}

pub enum SetCallError {
    TooManyCalls = 0xFF01,
}

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
        Transaction::Invoke(tx) => {
            tx.calls = Vec::with_capacity(nb as usize);
        }
        Transaction::DeployAccount(tx) => {
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
        Transaction::Invoke(tx) => set_calldata_invoke(data, p2, &mut tx.calls),
        Transaction::DeployAccount(tx) => {
            set_calldata_deploy_account(data, &mut tx.constructor_calldata)
        }
        Transaction::None => panic!("Invalid transaction type"),
    }
}

fn set_calldata_invoke(
    data: &[u8],
    p2: SetCallStep,
    calls: &mut Vec<Call>,
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
            let call = calls.get_mut(idx).unwrap();
            let iter = data.chunks(FIELD_ELEMENT_SIZE);
            for d in iter {
                call.calldata.push(d.into());
            }
            Ok(())
        }
    }
}

fn set_calldata_deploy_account(
    data: &[u8],
    constructor_calldata: &mut Vec<FieldElement>,
) -> Result<(), SetCallError> {
    let iter = data.chunks(FIELD_ELEMENT_SIZE);
    for d in iter {
        constructor_calldata.push(d.into());
    }
    Ok(())
}
