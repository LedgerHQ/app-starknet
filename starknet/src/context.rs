use crate::types::FieldElement;

extern crate alloc;
use alloc::vec::Vec;

#[derive(Default, Debug)]
pub struct Call {
    pub to: FieldElement,
    pub selector: FieldElement,
    pub calldata: Vec<FieldElement>,
}

impl Call {
    pub fn reset(&mut self) {
        self.to.clear();
        self.selector.clear();
        for c in self.calldata.iter_mut() {
            c.clear();
        }
    }
}

#[derive(Default, Debug)]
pub struct Transaction {
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
}

impl Transaction {
    pub fn reset(&mut self) {
        self.sender_address.clear();
        self.tip.clear();
        self.l1_gas_bounds.clear();
        self.l2_gas_bounds.clear();
        self.chain_id.clear();
        self.nonce.clear();
        self.data_availability_mode.clear();
        for c in self.paymaster_data.iter_mut() {
            c.clear();
        }
        for c in self.account_deployment_data.iter_mut() {
            c.clear();
        }
        for c in self.calls.iter_mut() {
            c.reset();
        }
    }
}

pub enum RequestType {
    Unknown,
    GetPubkey,
    #[cfg(feature = "signhash")]
    SignHash,
    SignTx,
}

#[derive(Default, Debug)]
pub struct Hash {
    /// tx hash digest (Poseidon)
    pub m_hash: FieldElement,
    /// signature r
    pub r: [u8; 32],
    /// signature s
    pub s: [u8; 32],
    /// parity of y-coordinate of R in ECDSA signature
    pub v: u8,
}

impl Hash {
    pub fn reset(&mut self) {
        self.m_hash.clear();
        self.r.fill(0);
        self.s.fill(0);
        self.v = 0;
    }
}

pub struct Ctx {
    pub req_type: RequestType,
    pub tx: Transaction,
    pub hash: Hash,
    pub bip32_path: [u32; 6],
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            tx: Transaction::default(),
            hash: Hash::default(),
            req_type: RequestType::Unknown,
            bip32_path: [0u32; 6],
        }
    }

    pub fn reset(&mut self) {
        self.req_type = RequestType::Unknown;
        self.bip32_path.fill(0);
        self.tx.reset();
        self.hash.reset();
    }
}
