use crate::crypto;
use crate::types::FieldElement;

extern crate alloc;
use alloc::vec::Vec;

#[derive(Default, Debug)]
pub struct Call {
    pub to: FieldElement,
    pub selector: FieldElement,
    pub calldata: Vec<FieldElement>,
}

#[derive(Default, Debug)]
pub struct InvokeTransactionV1 {
    pub version: FieldElement,
    pub sender_address: FieldElement,
    pub max_fee: FieldElement,
    pub chain_id: FieldElement,
    pub nonce: FieldElement,
    pub calls: Vec<Call>,
    pub hasher: crypto::pedersen::PedersenHasher,
    pub hasher_calldata: crypto::pedersen::PedersenHasher,
}

#[derive(Default, Debug)]
pub struct InvokeTransactionV3 {
    pub version: FieldElement,
    pub sender_address: FieldElement,
    pub tip: FieldElement,
    pub l1_gas_bounds: FieldElement,
    pub l2_gas_bounds: FieldElement,
    pub paymaster_data: Vec<FieldElement>,
    pub chain_id: FieldElement,
    pub nonce: FieldElement,
    pub data_availability_mode: FieldElement,
    pub account_deployment_data: Vec<FieldElement>,
    pub calls: Vec<Call>,
    pub hasher: crypto::poseidon::PoseidonHasher,
    pub hasher_calldata: crypto::poseidon::PoseidonHasher,
}

#[derive(Default, Debug)]
pub struct DeployAccountTransactionV1 {
    pub version: FieldElement,
    pub contract_address: FieldElement,
    pub max_fee: FieldElement,
    pub chain_id: FieldElement,
    pub nonce: FieldElement,
    pub class_hash: FieldElement,
    pub contract_address_salt: FieldElement,
    pub constructor_calldata: Vec<FieldElement>,
    pub hasher: crypto::pedersen::PedersenHasher,
}

#[derive(Default, Debug)]
pub struct DeployAccountTransactionV3 {
    pub version: FieldElement,
    pub contract_address: FieldElement,
    pub tip: FieldElement,
    pub l1_gas_bounds: FieldElement,
    pub l2_gas_bounds: FieldElement,
    pub paymaster_data: Vec<FieldElement>,
    pub chain_id: FieldElement,
    pub nonce: FieldElement,
    pub data_availability_mode: FieldElement,
    pub class_hash: FieldElement,
    pub contract_address_salt: FieldElement,
    pub constructor_calldata: Vec<FieldElement>,
    pub hasher: crypto::poseidon::PoseidonHasher,
}

#[derive(Default, Debug)]
pub enum Transaction {
    #[default]
    None,
    InvokeV1(InvokeTransactionV1),
    InvokeV3(InvokeTransactionV3),
    DeployAccountV1(DeployAccountTransactionV1),
    DeployAccountV3(DeployAccountTransactionV3),
}

impl Transaction {
    pub fn get_nb_received_calls(&self) -> usize {
        match self {
            Transaction::InvokeV1(tx) => tx.calls.len(),
            Transaction::InvokeV3(tx) => tx.calls.len(),
            Transaction::DeployAccountV1(_tx) => 1usize,
            Transaction::DeployAccountV3(_tx) => 1usize,
            Transaction::None => 0usize,
        }
    }

    pub fn get_nb_calls(&self) -> usize {
        match self {
            Transaction::InvokeV1(tx) => tx.calls.capacity(),
            Transaction::InvokeV3(tx) => tx.calls.capacity(),
            Transaction::DeployAccountV1(_tx) => 1usize,
            Transaction::DeployAccountV3(_tx) => 1usize,
            Transaction::None => 0usize,
        }
    }
}

pub enum RequestType {
    Unknown,
    GetPubkey,
    #[cfg(feature = "signhash")]
    SignHash,
    SignTx,
    SignTxV1,
    SignDeployAccount,
    SignDeployAccountV1,
}

#[derive(Default, Debug)]
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub v: u8,
}

#[cfg(any(target_os = "stax", target_os = "flex"))]
use ledger_device_sdk::nbgl::{NbglHomeAndSettings, NbglSpinner};

pub struct Ctx {
    pub req_type: RequestType,
    pub tx: Transaction,
    pub hash: FieldElement,
    pub signature: Signature,
    pub bip32_path: [u32; 6],
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    pub home: NbglHomeAndSettings,
    #[cfg(any(target_os = "stax", target_os = "flex"))]
    pub spinner: NbglSpinner,
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            req_type: RequestType::Unknown,
            tx: Transaction::default(),
            hash: FieldElement::default(),
            signature: Signature::default(),
            bip32_path: [0u32; 6],
            #[cfg(any(target_os = "stax", target_os = "flex"))]
            home: NbglHomeAndSettings::new(),
            #[cfg(any(target_os = "stax", target_os = "flex"))]
            spinner: NbglSpinner::new(),
        }
    }

    pub fn reset(&mut self) {
        self.req_type = RequestType::Unknown;
        self.tx = Transaction::default();
        self.hash = FieldElement::default();
        self.signature = Signature::default();
        self.bip32_path.fill(0);
    }
}
