use ethereum_types::U256;
use serde::Deserialize;
use std::fmt;

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
    Unknown,
}

impl From<Ins> for u8 {
    fn from(value: Ins) -> Self {
        match value {
            Ins::GetVersion => 0u8,
            Ins::GetPubkey => 1u8,
            Ins::SignHash => 2u8,
            Ins::SignTx => 3u8,
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
            4.. => Ins::Unknown,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Call {
    pub to: String,
    pub selector: String,
    pub calldata: Vec<String>,
}

impl From<&Call> for Vec<FieldElement> {
    fn from(c: &Call) -> Self {
        let mut v: Vec<FieldElement> = Vec::new();

        let to = FieldElement(U256::from_str_radix(&c.to, 16).unwrap());
        v.push(to);

        let selector = FieldElement(U256::from_str_radix(&c.selector, 16).unwrap());
        v.push(selector);

        for c in c.calldata.iter() {
            let data = FieldElement(U256::from_str_radix(c, 16).unwrap());
            v.push(data);
        }
        v
    }
}

#[derive(Deserialize, Debug)]
pub struct Tx {
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
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub felts: Vec<String>,
}
