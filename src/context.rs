
pub struct FieldElement {
    pub data: [u8; 32]
}

pub struct PubKey {
    /// x-coordinate (32), y-coodinate (32)
    raw_public_key: [u8; 64],  
    /// for public key derivation
    chain_code: [u8; 32]      
}

struct CallDataItem {
    name_len: u8,
    name: [char; 32],
    item: [u8; 32]    
}

struct CallData {
    callarray_length: u8,
    to: [u8; 32],
    entry_point_length: u8,
    entry_point: [u8; 32],
    selector: [u8; 32],
    data_offset: u8,
    data_length: u8,
    calldata_length: u8,
    calldata: [CallDataItem; 5]
} 

struct Transaction {
    sender_address: FieldElement,
    calldata: CallData,             
    max_fee: FieldElement,
    nonce: FieldElement,
    version: FieldElement,
    chain_id: FieldElement
} 

const MAX_TRANSACTION_LEN: usize = 510;

pub struct RawTransaction {
    /// raw transaction serialized
    raw_tx: [u8;MAX_TRANSACTION_LEN],
    /// actual length of raw transaction
    raw_tx_len: usize,
    /// structured transaction                    
    transaction: Transaction            
}

pub struct PedersenInput {
	ab: [u8; 64]
}

pub enum Item {
    Default,
    Pk(PubKey),
    Tx(RawTransaction),
    Pe(PedersenInput)
}

pub enum RequestType {
    Unknown,
    GetPubkey,     
    SignHash,
    SignTransaction,
	ComputePedersen
}

const MAX_DER_SIG_LEN: usize = 72;

pub struct HashInfo {
    /// message hash digest (Pedersen)
    pub m_hash: [u8; 32],
    /// transaction signature encoded in DER
    pub signature: [u8; MAX_DER_SIG_LEN],
    /// length of transaction signature
    pub signature_len: u8,
    /// parity of y-coordinate of R in ECDSA signature
    pub v: u8
}

pub struct Ctx {
    //state_e state;  /// state of the context
    pub req_type: RequestType,
    pub item: Item,
    pub hash: HashInfo,
    pub bip32_path: [u32; 6],
    pub bip32_path_len: u8,
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            item: Item::Default,
            hash: HashInfo { m_hash: [0xFF; 32], signature: [0; MAX_DER_SIG_LEN], signature_len: 0, v: 0 },
            req_type: RequestType::Unknown,
            bip32_path: [0u32; 6],
            bip32_path_len: 0
        }
    }
}