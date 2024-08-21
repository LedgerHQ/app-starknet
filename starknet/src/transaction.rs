use crate::{context::Call, context::Transaction, types::FieldElement};

extern crate alloc;
use alloc::vec::Vec;

const FIELD_ELEMENT_SIZE: usize = 32;

pub fn set_tx_fields(data: &[u8], tx: &mut Transaction) {
    let mut iter = data.chunks(FIELD_ELEMENT_SIZE);

    tx.sender_address = iter.next().unwrap().into();
    tx.tip = iter.next().unwrap().into();
    tx.l1_gas_bounds = iter.next().unwrap().into();
    tx.l2_gas_bounds = iter.next().unwrap().into();
    tx.chain_id = iter.next().unwrap().into();
    tx.nonce = iter.next().unwrap().into();
    tx.data_availability_mode = iter.next().unwrap().into();
}

// For future use: currently paymaster_data is always empty.
// See https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v3_transaction_fields
pub fn set_paymaster_data(_data: &[u8], _p2: u8, _paymaster_data: &mut [FieldElement]) {}

// For future use: currently account_deployment_data is always empty.
// See https://docs.starknet.io/architecture-and-concepts/network-architecture/transactions/#v3_transaction_fields
pub fn set_account_deployment_data(
    _data: &[u8],
    _p2: u8,
    _account_deployment_data: &mut [FieldElement],
) {
}

pub enum SetCallError {
    TooManyCalls = 0xFF01,
}

pub enum SetCallStep {
    New = 0x00,
    Add = 0x01,
}

impl From<u8> for SetCallStep {
    fn from(value: u8) -> Self {
        match value {
            0x00 => SetCallStep::New,
            0x01 => SetCallStep::Add,
            _ => panic!("Invalid SetCallStep value"),
        }
    }
}

pub fn set_call(data: &[u8], p2: SetCallStep, calls: &mut Vec<Call>) -> Result<(), SetCallError> {
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
        SetCallStep::Add => {
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
