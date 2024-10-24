use ethereum_types::U256;
use serde::Deserialize;
use starknet::core::utils::starknet_keccak;
use std::fmt;
use std::vec::Vec;

const DEFAULT_ENTRY_POINT_NAME: &str = "__default__";
const DEFAULT_L1_ENTRY_POINT_NAME: &str = "__l1_default__";

#[derive(Copy, Clone, Debug)]
pub struct FieldElement(pub U256);

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = [0u8; 32];
        self.0.to_big_endian(&mut s[..]);
        for b in s {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl TryFrom<FieldElement> for [u8; 32] {
    type Error = ();
    fn try_from(fe: FieldElement) -> Result<Self, Self::Error> {
        let mut s = [0u8; 32];
        fe.0.to_big_endian(&mut s[..]);
        Ok(s)
    }
}

impl TryFrom<&str> for FieldElement {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.starts_with("0x") {
            true => Ok(FieldElement(U256::from_str_radix(s, 16).unwrap())),
            false => Ok(FieldElement(U256::from_str_radix(s, 10).unwrap())),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Ins {
    GetVersion,
    GetPubkey,
    SignHash,
    SignTx,
    SignTxV1,
    SignDeployAccount,
    SignDeployAccountV1,
    Unknown,
}

impl From<Ins> for u8 {
    fn from(value: Ins) -> Self {
        match value {
            Ins::GetVersion => 0u8,
            Ins::GetPubkey => 1u8,
            Ins::SignHash => 2u8,
            Ins::SignTx => 3u8,
            Ins::SignTxV1 => 4u8,
            Ins::SignDeployAccount => 5u8,
            Ins::SignDeployAccountV1 => 6u8,
            Ins::Unknown => 0xff,
        }
    }
}

impl From<u8> for Ins {
    fn from(v: u8) -> Self {
        match v {
            0 => Ins::GetVersion,
            1 => Ins::GetPubkey,
            2 => Ins::SignHash,
            3 => Ins::SignTx,
            4 => Ins::SignTxV1,
            5 => Ins::SignDeployAccount,
            6 => Ins::SignDeployAccountV1,
            7.. => Ins::Unknown,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Call {
    pub to: String,
    pub entrypoint: String,
    pub calldata: Vec<String>,
}

impl From<&Call> for Vec<FieldElement> {
    fn from(c: &Call) -> Self {
        let mut v: Vec<FieldElement> = Vec::new();

        let to = FieldElement(U256::from_str_radix(&c.to, 16).unwrap());
        v.push(to);

        let selector = get_selector_from_name(&c.entrypoint);

        v.push(selector);

        for c in c.calldata.iter() {
            let data = FieldElement(U256::from_str_radix(c, 16).unwrap());
            v.push(data);
        }
        v
    }
}

#[derive(Deserialize, Debug)]
pub struct InvokeV3 {
    pub url: String,
    pub version: u8,
    pub sender_address: String,
    pub tip: String,
    pub l1_gas_bounds: String,
    pub l2_gas_bounds: String,
    pub paymaster_data: Vec<String>,
    pub chain_id: String,
    pub nonce: String,
    pub data_availability_mode: String,
    pub account_deployment_data: Vec<String>,
    pub calls: Vec<Call>,
    pub dpath: String,
}

#[derive(Deserialize, Debug)]
pub struct InvokeV1 {
    pub url: String,
    pub version: u8,
    pub sender_address: String,
    pub max_fee: String,
    pub chain_id: String,
    pub nonce: String,
    pub calls: Vec<Call>,
    pub dpath: String,
}

pub enum Tx {
    V1(InvokeV1),
    V3(InvokeV3),
    DeployV1(DeployAccountV1),
    DeployV3(DeployAccountV3),
}

#[derive(Deserialize, Debug)]
pub struct DeployAccountV3 {
    pub url: String,
    pub version: u8,
    pub contract_address: String,
    pub tip: String,
    pub l1_gas_bounds: String,
    pub l2_gas_bounds: String,
    pub paymaster_data: Vec<String>,
    pub chain_id: String,
    pub nonce: String,
    pub data_availability_mode: String,
    pub class_hash: String,
    pub contract_address_salt: String,
    pub constructor_calldata: Vec<String>,
    pub dpath: String,
}

#[derive(Deserialize, Debug)]
pub struct DeployAccountV1 {
    pub url: String,
    pub version: u8,
    pub contract_address: String,
    pub max_fee: String,
    pub chain_id: String,
    pub nonce: String,
    pub class_hash: String,
    pub contract_address_salt: String,
    pub constructor_calldata: Vec<String>,
    pub dpath: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub felts: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Hash {
    pub dpath: String,
    pub hash: String,
}

#[derive(Deserialize, Debug)]
pub struct Dpath {
    pub dpath: String,
}

pub fn get_selector_from_name(func_name: &str) -> FieldElement {
    if func_name == DEFAULT_ENTRY_POINT_NAME || func_name == DEFAULT_L1_ENTRY_POINT_NAME {
        FieldElement(U256::from(0u8))
    } else {
        let name_bytes = func_name.as_bytes();

        let selector = starknet_keccak(name_bytes);

        FieldElement(U256::from_str_radix(selector.to_fixed_hex_string().as_str(), 16).unwrap())
    }
}
