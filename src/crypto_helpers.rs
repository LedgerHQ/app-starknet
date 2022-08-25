use nanos_sdk::ecc::{Stark256, ECPublicKey};
use nanos_sdk::io::SyscallError;

pub const EIP2645_PATH_LENGTH: usize = 6;
/// Length in bytes of an EIP-2645 derivation path (without m), e.g m/2645'/1195502025'/1148870696'/0'/0'/0
/// with every step encoded with 4 bytes (total length = 6 x 4 = 24 bytes)
pub const EIP2645_PATH_BYTES_LENGTH: usize = 24;
/// Hardened 2645 value
pub const EIP2645_PATH_PREFIX: u32 = 2147486293;   

#[derive(Debug)]
pub enum HelperError {
    UnvalidPathError,
    SignError,
    GenericError,
}

/// Helper function that signs with ECDSA in deterministic nonce
pub fn detecdsa_sign(
    bytes_path: &[u8],
    m: &[u8]
) -> Result<([u8; 32], [u8; 32]) , HelperError> {

    let mut path = [0u32; EIP2645_PATH_LENGTH];
    get_derivation_path(bytes_path, &mut path[..]).unwrap();

    match Stark256::from_path(&path[..]).deterministic_sign(m) {
        Ok(s) => {
            let der = s.0;
            let mut r = [0u8; 32];
            let mut s = [0u8; 32];
            convert_der_to_rs(&der[..], &mut r, &mut s).unwrap();
            Ok((r, s))
        },
        Err(_) => Err(HelperError::SignError)
    }
}

/// Helper function that retrieves public key
pub fn get_pubkey(bytes_path: &[u8]) -> Result<ECPublicKey<65, 'W'>, SyscallError> {

    let mut path = [0u32; EIP2645_PATH_LENGTH];
    get_derivation_path(bytes_path, &mut path[..]).unwrap();

    let private_key = Stark256::from_path(&path[..]);

    match private_key.public_key() {
        Ok(public_key) => Ok(public_key),
        Err(_) => Err(SyscallError::Unspecified)
    }
}

fn get_derivation_path(buf: &[u8], path: &mut [u32]) -> Result<(),  HelperError>  {

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
                EIP2645_PATH_PREFIX => Ok(()),
                _ => Err(HelperError::UnvalidPathError),
            }
        },
        _ => Err(HelperError::UnvalidPathError),
    }
}

#[derive(Debug)]
enum ConvertError<const R: usize, const S: usize> {
    /// The DER prefix (at index 0) found was different than the expected 0x30
    InvalidDERPrefix(u8),
    /// The R marker was different than expected (0x02)
    InvalidRMarker(u8),
    /// The encoded len for R was not the same as the expected
    InvalidRLen(usize),
    /// The S marker was different than expected (0x02)
    InvalidSMarker(u8),
    /// The encoded len for S was not the same as the expected
    InvalidSLen(usize),
    /// Passed signature was too short to be read properly
    TooShort,
    /// Passed signature encoded payload len was not in the expected range
    InvalidPayloadLen {
        min: usize,
        payload: usize,
        max: usize,
    },
}

/// Converts a DER encoded signature into a (r, s) encoded signture
fn convert_der_to_rs<const R: usize, const S: usize>(
    sig: &[u8],
    out_r: &mut [u8; R],
    out_s: &mut [u8; S],
) -> Result<(), ConvertError<R, S>> {
    const MINPAYLOADLEN: usize = 1;
    const PAYLOADLEN: usize = 32;
    const MAXPAYLOADLEN: usize = 33;

    let payload_range = core::ops::RangeInclusive::new(MINPAYLOADLEN, MAXPAYLOADLEN);
    // https://github.com/libbitcoin/libbitcoin-system/wiki/ECDSA-and-DER-Signatures#serialised-der-signature-sequence
    // 0                [1 byte]   - DER Prefix
    // 1                [1 byte]   - Payload len
    // 2                [1 byte]   - R Marker. Always 02
    // 3                [1 byte]   - R Len                      RLEN
    // ROFFSET ...      [.?. byte] - R                          ROFFSET
    // ROFFSET+RLEN     [1 byte]   - S Marker. Always 02
    // ROFFSET+RLEN+1   [1 byte]   - S Length                   SLEN
    // ROFFSET+RLEN+2   [.?. byte] - S                          SOFFSET

    //check that we have at least the DER prefix and the payload len
    if sig.len() < 2 {
        return Err(ConvertError::TooShort);
    }

    //check DER prefix
    if sig[0] != 0x30 {
        return Err(ConvertError::InvalidDERPrefix(sig[0]));
    }

    //check payload len size
    let payload_len = sig[1] as usize;
    let min_payload_len = 2 + MINPAYLOADLEN + 2 + MINPAYLOADLEN;
    let max_payload_len = 2 + MAXPAYLOADLEN + 2 + MAXPAYLOADLEN;
    if payload_len < min_payload_len || payload_len > max_payload_len {
        return Err(ConvertError::InvalidPayloadLen {
            min: min_payload_len,
            payload: payload_len,
            max: max_payload_len,
        });
    }

    //check that the input slice is at least as long as the encoded len
    if sig.len() - 2 < payload_len {
        return Err(ConvertError::TooShort);
    }

    //retrieve R
    if sig[2] != 0x02 {
        return Err(ConvertError::InvalidRMarker(sig[2]));
    }

    let r_len = sig[3] as usize;
    if !payload_range.contains(&r_len) {
        return Err(ConvertError::InvalidRLen(r_len));
    }

    //sig[4], after DER, after Payload, after marker after len
    let r = &sig[4..4 + r_len];

    //retrieve S
    if sig[4 + r_len] != 0x02 {
        return Err(ConvertError::InvalidSMarker(sig[4 + r_len]));
    }

    let s_len = sig[4 + r_len + 1] as usize;
    if !payload_range.contains(&r_len) {
        return Err(ConvertError::InvalidSLen(s_len));
    }

    //after r (4 + r_len), after marker, after len
    let s = &sig[4 + r_len + 2..4 + r_len + 2 + s_len];

    out_r.fill(0);
    out_r[PAYLOADLEN - r_len..].copy_from_slice(r);

    out_s.fill(0);
    out_s[PAYLOADLEN - s_len..].copy_from_slice(s);

    Ok(())
}
