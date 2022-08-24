use nanos_sdk::ecc::{Stark256, make_bip32_path, ECPublicKey};
use nanos_sdk::io::SyscallError;

pub const BIP32_PATH: [u32; 6] = make_bip32_path(b"m/2645'/1195502025'/1148870696'/0'/0'/0");

/// Length of an EIP-2645 derivation path (without m), e.g m/2645'/1195502025'/1148870696'/0'/0'/0
pub const EIP2645_PATH_LENGTH: u8 = 6;
pub const EIP2645_PATH_BYTES_LENGTH: usize = 24;
/// Hardened 2645 value
pub const EIP2645_PATH_PREFIX: u32 = 2147486293;   

#[derive(Debug)]
pub enum HelperError {
    UnvalidPath,
    GenericError,
}

/// Helper function that signs with ECDSA in deterministic nonce
pub fn detecdsa_sign(
    m: &[u8]
) -> Result<([u8; 72], u32), u32> {
    Ok(Stark256::from_path(&BIP32_PATH).deterministic_sign(m)?)
}

pub fn get_pubkey(derivation_path: &[u32]) -> Result<ECPublicKey<65, 'W'>, SyscallError> {
    
    let private_key = Stark256::from_path(derivation_path);

    match private_key.public_key() {
        Ok(public_key) => Ok(public_key),
        Err(_) => Err(SyscallError::Unspecified)
    }
}

pub fn get_derivation_path(buf: &[u8], path: &mut [u32]) -> Result<u8,  HelperError>  {

    match buf.len() {
        EIP2645_PATH_BYTES_LENGTH => {
            let mut j = 0;
            for i in 0..5 {
                path[i] += (buf[j] as u32) << 24;
                path[i] += (buf[j + 1] as u32) << 16;
                path[i] += (buf[j + 2] as u32) << 8;
                path[i] += buf[j + 3] as u32;
                j += 4;
            }
            match path[0] {
                EIP2645_PATH_PREFIX => Ok(0),
                _ => Err(HelperError::UnvalidPath),
            }
        },
        _ => Err(HelperError::UnvalidPath),
    }
}
