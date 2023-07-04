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

#[derive(Debug, Clone, Copy, Default)]
pub struct Signature {
     /// signature r 
     pub r: [u8; 32],
     /// signature s 
     pub s: [u8; 32],
     /// parity of y-coordinate of R in ECDSA signature
     pub v: u8
}

pub struct Ctx {
    pub req_type: RequestType,
    pub tx_info: TransactionInfo,
    pub is_bettermulticall: bool,
    pub is_first_loop: bool,
    pub call: Call,
    pub a_call: AbstractCall,
    pub nb_call_rcv: usize,
    pub call_to_nref: [u8; 8],
    pub call_to_string: [String<64>; 8],
    pub hash: FieldElement,
    pub bip32_path: [u32; 6],
    pub bip32_path_len: u8,
    pub plugin_internal_ctx: [u8; 255],
    pub num_ui_screens: u8,
    pub signature: Signature
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            tx_info: TransactionInfo::new(),
            is_bettermulticall: false,
            is_first_loop: false,
            call: Call::new(),
            a_call: AbstractCall::new(),
            nb_call_rcv: 0,
            call_to_nref: [0u8; 8],
            call_to_string: [String::<64>::new(); 8],
            hash: Default::default(),
            req_type: RequestType::Unknown,
            bip32_path: [0u32; 6],
            bip32_path_len: 0,
            plugin_internal_ctx: [0u8; 255],
            num_ui_screens: 0,
            signature: Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.req_type = RequestType::Unknown;
        self.tx_info.clear();
        self.is_bettermulticall = false;
        self.is_first_loop = false;
        self.call.clear();
        self.a_call.clear();
        self.nb_call_rcv = 0;
        self.call_to_nref = [0u8; 8];
        self.call_to_string = [String::<64>::new(); 8];
        self.hash = Default::default();
        self.bip32_path.fill(0);
        self.bip32_path_len = 0;
        self.plugin_internal_ctx.fill(0);
        self.num_ui_screens = 0;
        self.signature = Default::default()
    }
}