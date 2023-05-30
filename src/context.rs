use nanos_sdk::starknet::{FieldElement, Transaction};

pub enum RequestType {
    Unknown,
    GetPubkey,     
    SignHash,
    SignTransaction,
	ComputePedersen,
    TestPlugin
}

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

    pub fn clear(&mut self) {
        self.m_hash.clear();
        self.calldata_hash.clear();
        self.r.fill(0);
        self.s.fill(0);
        self.v = 0;
    }
}

pub struct Ctx {
    //state_e state;  /// state of the context
    pub req_type: RequestType,
    pub tx_info: Transaction,
    pub hash_info: HashInfo,
    pub bip32_path: [u32; 6],
    pub bip32_path_len: u8,
    pub plugin_internal_ctx: [u8; 255],
    pub plugin_internal_ctx_len: usize,
    pub num_ui_screens: u8
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            tx_info: Transaction::new(),
            hash_info: HashInfo::new(),
            req_type: RequestType::Unknown,
            bip32_path: [0u32; 6],
            bip32_path_len: 0,
            plugin_internal_ctx: [0u8; 255],
            plugin_internal_ctx_len: 0,
            num_ui_screens: 0
        }
    }

    pub fn clear(&mut self) {
        self.req_type = RequestType::Unknown;
        self.bip32_path.fill(0);
        self.bip32_path_len = 0;
        self.tx_info.clear();
        self.hash_info.clear();
        self.plugin_internal_ctx.fill(0);
        self.plugin_internal_ctx_len = 0;
        self.num_ui_screens = 0;
    }
}