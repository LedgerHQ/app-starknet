extern crate alloc;
use alloc::string::{String, ToString};
use core::cmp::Ordering;
use core::ops::{Add, AddAssign, Div, Mul, Rem, Sub};
use ledger_secure_sdk_sys::*;
use num_bigint::BigUint;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, PartialOrd)]
pub struct FieldElement {
    pub value: [u8; 32],
}

impl FieldElement {
    pub const INVOKE: FieldElement = FieldElement {
        value: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x69, 0x6e,
            0x76, 0x6f, 0x6b, 0x65,
        ],
    };

    pub const ZERO: FieldElement = FieldElement { value: [0u8; 32] };

    pub const ONE: FieldElement = FieldElement {
        value: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ],
    };

    pub const TWO: FieldElement = FieldElement {
        value: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x02,
        ],
    };

    pub const THREE: FieldElement = FieldElement {
        value: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x03,
        ],
    };

    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.value.fill(0);
    }

    pub fn copy_from(&mut self, f: &FieldElement) {
        self.value.copy_from_slice(&f.value);
    }

    pub fn div_rem(&self, other: &FieldElement) -> (FieldElement, FieldElement) {
        let remainder = *self % *other;
        ((*self - remainder) / *other, remainder)
    }

    pub fn inverse(&self) -> FieldElement {
        let mut res = FieldElement::default();

        unsafe {
            cx_math_invprimem_no_throw(
                res.value.as_mut_ptr(),
                self.value.as_ptr(),
                P.value.as_ptr(),
                32,
            );
        }

        res
    }

    pub fn to_dec_string(&self, decimals: Option<usize>) -> String {
        let bn = BigUint::from_bytes_be(self.value.as_ref());
        match decimals {
            Some(d) => {
                let bn_str = bn.to_string();
                let len = bn_str.len();
                if len <= d {
                    let mut s = String::from("0.");
                    s.push_str(&"0".repeat(d - len));
                    s.push_str(&bn_str);
                    s
                } else {
                    let (int_part, dec_part) = bn_str.split_at(len - d);
                    let mut s = String::from(int_part);
                    s.push_str(".");
                    s.push_str(dec_part);
                    s
                }
            }
            None => bn.to_string(),
        }
    }

    pub fn to_hex_string(&self) -> String {
        hex::encode(&self.value)
    }
}

// P is the Starknet 252 Prime
pub const P: FieldElement = FieldElement {
    value: [
        0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01,
    ],
};

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = FieldElement::default();

        unsafe {
            cx_math_addm_no_throw(
                res.value.as_mut_ptr(),
                self.value.as_ptr(),
                other.value.as_ptr(),
                P.value.as_ptr(),
                32,
            );
        }
        res
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut res = FieldElement::default();

        unsafe {
            cx_math_multm_no_throw(
                res.value.as_mut_ptr(),
                self.value.as_ptr(),
                other.value.as_ptr(),
                P.value.as_ptr(),
                32,
            );
        }
        res
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut res = FieldElement::default();

        unsafe {
            cx_math_subm_no_throw(
                res.value.as_mut_ptr(),
                self.value.as_ptr(),
                other.value.as_ptr(),
                P.value.as_ptr(),
                32,
            );
        }
        res
    }
}

impl Rem for FieldElement {
    type Output = Self;

    fn rem(mut self, other: Self) -> Self {
        unsafe {
            cx_math_modm_no_throw(self.value.as_mut_ptr(), 32, other.value.as_ptr(), 32);
        }
        self
    }
}

impl Ord for FieldElement {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut result: i32 = 0;

        unsafe {
            cx_math_cmp_no_throw(self.value.as_ptr(), other.value.as_ptr(), 32, &mut result);
        }

        match result {
            r if r < 0 => Ordering::Less,
            r if r > 0 => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let other_inverse = other.inverse();

        // Use the multiplication method defined earlier
        self * other_inverse
    }
}

impl AddAssign for FieldElement {
    fn add_assign(&mut self, other: Self) {
        unsafe {
            let value = self.value;
            cx_math_addm_no_throw(
                self.value.as_mut_ptr(),
                value.as_ptr(),
                other.value.as_ptr(),
                P.value.as_ptr(),
                32,
            );
        }
    }
}

impl From<&[u8]> for FieldElement {
    fn from(data: &[u8]) -> Self {
        let mut value: [u8; 32] = [0; 32];
        value[32 - data.len()..].copy_from_slice(data);
        Self { value: value }
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

// assumes usize < FieldElement (should be true, especially on the nano)
impl From<usize> for FieldElement {
    fn from(num: usize) -> Self {
        let mut f = FieldElement::new();
        let size_of_usize = core::mem::size_of::<usize>();
        let offset = if size_of_usize >= f.value.len() {
            0
        } else {
            f.value.len() - size_of_usize
        };

        for i in 0..size_of_usize {
            f.value[offset + i] = (num >> ((size_of_usize - 1 - i) * 8)) as u8;
        }

        f
    }
}

impl From<FieldElement> for usize {
    fn from(fe: FieldElement) -> usize {
        let mut value: usize = 0;
        let size_of_usize = core::mem::size_of::<usize>();
        let offset = if size_of_usize >= fe.value.len() {
            0
        } else {
            fe.value.len() - size_of_usize
        };

        for i in 0..size_of_usize {
            value |= (fe.value[i + offset] as usize) << ((size_of_usize - 1 - i) * 8);
        }

        value
    }
}

impl From<&str> for FieldElement {
    fn from(data: &str) -> Self {
        let mut fe = FieldElement::default();
        if data.len() != 64 {
            panic!("Invalid hex string length for FieldElement");
        }
        match hex::decode_to_slice(data, &mut fe.value[..]) {
            Ok(_) => fe,
            Err(_) => FieldElement::default(),
        }
    }
}