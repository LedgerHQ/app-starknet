use starknet_sdk::types::{
    FieldElement,
    TransactionInfo,
    Call, 
    AbstractCall
};

use nanos_sdk::string::String;

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
    pub req_type: RequestType,
    pub tx_info: TransactionInfo,
    pub is_bettermulticall: bool,
    pub is_first_loop: bool,
    pub call: Call,
    pub a_call: AbstractCall,
    pub nb_calls_rcv: usize,
    pub call_to_nref: [u8; 16],
    pub call_to_string: [String<32>; 16],
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
            tx_info: TransactionInfo::new(),
            is_bettermulticall: false,
            is_first_loop: false,
            call: Call::new(),
            a_call: AbstractCall::new(),
            nb_calls_rcv: 0,
            call_to_nref: [0u8; 16],
            call_to_string: [String::<32>::new(); 16],
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
        self.tx_info.clear();
        self.is_bettermulticall = false;
        self.is_first_loop = false;
        self.call.clear();
        self.a_call.clear();
        self.nb_calls_rcv = 0;
        self.call_to_nref = [0u8; 16];
        self.call_to_string = [String::<32>::new(); 16];
        self.hash_info.clear();
        self.bip32_path.fill(0);
        self.bip32_path_len = 0;
        self.plugin_internal_ctx.fill(0);
        self.plugin_internal_ctx_len = 0;
        self.num_ui_screens = 0;
    }
}