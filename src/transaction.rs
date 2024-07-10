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

pub fn set_paymaster_data(_data: &[u8], _p2: u8, _paymaster_data: &mut Vec<FieldElement>) {}

pub fn set_account_deployment_data(
    _data: &[u8],
    _p2: u8,
    _account_deployment_data: &mut Vec<FieldElement>,
) {
}

pub fn set_call(data: &[u8], p2: u8, calls: &mut Vec<Call>) {
    match p2 {
        0x00 => {
            if calls.len() == calls.capacity() {
                panic!("Too many calls");
            }
            let mut call = Call::default();
            let mut iter = data.chunks(FIELD_ELEMENT_SIZE);

            call.to = iter.next().unwrap().into();
            call.selector = iter.next().unwrap().into();
            for d in iter {
                call.calldata.push(d.into());
            }
            calls.push(call);
        }
        0x01 => {
            let idx = calls.len() - 1;
            let call = calls.get_mut(idx).unwrap();
            let iter = data.chunks(FIELD_ELEMENT_SIZE);
            for d in iter {
                call.calldata.push(d.into());
            }
        }
        _ => {}
    }
}
