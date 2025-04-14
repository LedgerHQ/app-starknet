use serde::Deserialize;
use starknet::core::utils::get_selector_from_name;
use starknet_types_core::felt::Felt;
use std::vec::Vec;

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

impl From<&Call> for Vec<Felt> {
    fn from(c: &Call) -> Self {
        let mut v: Vec<Felt> = Vec::new();

        let to = Felt::from_hex_unchecked(&c.to);
        v.push(to);

        let selector = get_selector_from_name(&c.entrypoint).unwrap();
        v.push(selector);

        let calldata_length = c.calldata.len();
        v.push(Felt::from(calldata_length));

        c.calldata.iter().for_each(|c| {
            let data = Felt::from_hex_unchecked(c);
            v.push(data);
        });
        v
    }
}

#[derive(Deserialize, Debug)]
pub struct Fee {
    pub max_amount: String,
    pub max_price_per_unit: String,
}

#[derive(Deserialize, Debug)]
pub struct ResourceBounds {
    pub l2_gas: Fee,
    pub l1_gas: Fee,
    pub l1_data_gas: Fee,
}

#[derive(Deserialize, Debug)]
pub struct InvokeV3 {
    pub version: String,
    pub sender_address: String,
    pub tip: String,
    pub resource_bounds: ResourceBounds,
    pub paymaster_data: Vec<String>,
    pub chain_id: String,
    pub nonce: String,
    pub data_availability_mode: String,
    pub account_deployment_data: Vec<String>,
    pub calls: Vec<Call>,
}

#[derive(Deserialize, Debug)]
pub struct InvokeV1 {
    pub version: String,
    pub sender_address: String,
    pub max_fee: String,
    pub chain_id: String,
    pub nonce: String,
    pub calls: Vec<Call>,
}

pub enum Tx {
    V1(InvokeV1),
    V3(InvokeV3),
    DeployV1(DeployAccountV1),
    DeployV3(DeployAccountV3),
}

#[derive(Deserialize, Debug)]
pub struct DeployAccountV3 {
    pub version: String,
    pub contract_address: String,
    pub tip: String,
    pub resource_bounds: ResourceBounds,
    pub paymaster_data: Vec<String>,
    pub chain_id: String,
    pub nonce: String,
    pub data_availability_mode: String,
    pub class_hash: String,
    pub contract_address_salt: String,
    pub constructor_calldata: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct DeployAccountV1 {
    pub version: String,
    pub contract_address: String,
    pub max_fee: String,
    pub chain_id: String,
    pub nonce: String,
    pub class_hash: String,
    pub contract_address_salt: String,
    pub constructor_calldata: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub felts: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Hash {
    pub dpath_signhash: String,
    pub hash: String,
}

#[derive(Deserialize, Debug)]
pub struct Dpath {
    pub dpath_getpubkey: String,
}
