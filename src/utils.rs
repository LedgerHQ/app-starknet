use core::char;

/// Convert to hex. Returns a static buffer of 64 bytes
#[inline]
pub fn to_hex<const N: usize>(m: &[u8]) -> Result<[u8; N], ()> {
    if 2 * m.len() > N {
        return Err(());
    }
    let mut hex = [0u8; N];
    let mut i = 0;
    for c in m {
        let c0 = char::from_digit((c >> 4).into(), 16).unwrap();
        let c1 = char::from_digit((c & 0xf).into(), 16).unwrap();
        hex[i] = c0 as u8;
        hex[i + 1] = c1 as u8;
        i += 2;
    }
    Ok(hex)
}


#[cfg(feature = "speculos")]
pub mod print {

    use nanos_sdk::testing::debug_print;
    use crate::context::FieldElement;

    pub fn printf(s: &str) {
        debug_print(s);
    }
    
    pub fn printf_slice<const N: usize>(tab: &[u8]) {
        let hex: [u8; N] = super::to_hex(tab).unwrap();
        let m = core::str::from_utf8(&hex).unwrap();
        debug_print(m);
    }

    pub fn printf_fe(prefix: &str, val: &FieldElement) {
        printf(prefix);
        printf_slice::<64>(&val.value[..]);
        printf("\n");
    }
}

#[cfg(feature = "device")]
pub mod print {

    use crate::context::FieldElement;

    pub fn printf(_s: &str) {}
    pub fn printf_slice<const N: usize>(_tab: &[u8]) {}
    pub fn printf_fe(_prefix: &str, _val: &FieldElement) {}
}