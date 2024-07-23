#[derive(Debug)]
pub struct TokenInfo {
    pub address: &'static str,
    pub ticker: &'static str,
    pub decimals: usize,
}

pub const NB_ERC20_TOKENS: usize = 3;

pub const ERC20_TOKENS: [TokenInfo; NB_ERC20_TOKENS] = [
    TokenInfo {
        address: "068f5c6a61780768455de69077e07e89787839bf8166decfbf92b645209c0fb8",
        ticker: "USDT",
        decimals: 6,
    },
    TokenInfo {
        address: "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        ticker: "ETH",
        decimals: 18,
    },
    TokenInfo {
        address: "04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d",
        ticker: "STRK",
        decimals: 18,
    },
];

pub const TRANSFER: &str = "0083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e";
//pub const APPROVE: &str = "0219209e083275171774dab1df80982e9df2096516f06319c5c6d71ae0a8480c";
