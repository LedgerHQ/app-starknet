#[derive(Debug, Copy, Clone)]
pub struct FieldElement {
    pub value: [u8; 32]
}

impl FieldElement {

    pub const INVOKE: FieldElement = FieldElement {
        value: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x69, 0x6e, 0x76, 0x6f, 0x6b, 0x65
        ]
    };

    pub const ZERO: FieldElement = FieldElement {
        value: [0u8; 32]
    };

    pub fn new() -> Self {
        Self {
            value: [0u8; 32]
        }
    }
}

impl From<&[u8]> for FieldElement {
    fn from(data: &[u8]) -> Self {
        let mut value: [u8; 32] = [0; 32];
        value.copy_from_slice(data); 
        Self {
            value: value
        }
    }
}

impl From<u8> for FieldElement {
    fn from(data: u8) -> Self {
        let mut f = FieldElement::new();
        f.value[31] = data;
        f
    }
}

impl From<FieldElement> for u8 {
    fn from(fe: FieldElement) -> u8 {
        fe.value[31]
    }
}

pub struct PubKey {
    /// x-coordinate (32), y-coodinate (32)
    raw_public_key: [u8; 64],  
    /// for public key derivation
    chain_code: [u8; 32]      
}

#[derive(Debug, Copy, Clone)]
pub struct CallArray {
    pub to: FieldElement,
    pub entry_point_length: u8,
    pub entry_point: [u8; 32],
    pub selector: FieldElement,
    pub data_offset: FieldElement,
    pub data_len: FieldElement,
}

impl CallArray {
    pub fn new() -> Self {
        Self {
            to: FieldElement::new(),
            entry_point_length: 0,
            entry_point: [0u8; 32],
            selector: FieldElement::new(),
            data_offset: FieldElement::new(),
            data_len: FieldElement::new() 
        }
    }
}

pub struct CallData {
    pub call_array_len: FieldElement,
    pub calls: [CallArray; 10],
    pub calldata_len: FieldElement,
} 

impl CallData {
    pub fn new() -> Self {
        Self {
            call_array_len: FieldElement::new(),
            calls: [CallArray::new(); 10],
            calldata_len: FieldElement::new()
        }
    }
}

pub struct Transaction {
    pub sender_address: FieldElement,
    pub calldata: CallData,             
    pub max_fee: FieldElement,
    pub nonce: FieldElement,
    pub version: FieldElement,
    pub chain_id: FieldElement
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            sender_address: FieldElement::new(),
            calldata: CallData::new(),
            max_fee: FieldElement::new(),
            nonce: FieldElement::new(),
            version: FieldElement::new(),
            chain_id: FieldElement::new()
        }
    }
}

const MAX_TRANSACTION_LEN: usize = 510;

/*pub struct RawTransaction {
    /// raw transaction serialized
    raw_tx: [u8;MAX_TRANSACTION_LEN],
    /// actual length of raw transaction
    raw_tx_len: usize,
    /// structured transaction                    
    transaction: Transaction            
}*/

pub struct PedersenInput {
	ab: [u8; 64]
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
    pub m_hash: FieldElement,
    /// calldata_hash
    pub calldata_hash: FieldElement,
    /// signature r 
    pub r: [u8; 32],
    /// signature s 
    pub s: [u8; 32],
    /// parity of y-coordinate of R in ECDSA signature
    pub v: u8
}

impl HashInfo {
    pub fn new() -> Self {
        Self {
            m_hash: FieldElement::new(),
            calldata_hash: FieldElement::new(),
            r: [0u8; 32],
            s: [0u8; 32],
            v: 0
        }
    }
}

pub struct Ctx {
    //state_e state;  /// state of the context
    pub req_type: RequestType,
    pub tx_info: Transaction,
    pub hash_info: HashInfo,
    pub bip32_path: [u32; 6],
    pub bip32_path_len: u8,
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            tx_info: Transaction::new(),
            hash_info: HashInfo::new(),
            req_type: RequestType::Unknown,
            bip32_path: [0u32; 6],
            bip32_path_len: 0
        }
    }
}