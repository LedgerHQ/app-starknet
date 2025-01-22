use crate::types::{FieldElement, P};
use ledger_secure_sdk_sys::*;

/* EC points */
struct ECPoint {
    x: [u8; 32],
    y: [u8; 32],
}

const PEDERSEN_SHIFT: ECPoint = ECPoint {
    x: [
        0x04, 0x9e, 0xe3, 0xeb, 0xa8, 0xc1, 0x60, 0x07, 0x00, 0xee, 0x1b, 0x87, 0xeb, 0x59, 0x9f,
        0x16, 0x71, 0x6b, 0x0b, 0x10, 0x22, 0x94, 0x77, 0x33, 0x55, 0x1f, 0xde, 0x40, 0x50, 0xca,
        0x68, 0x04,
    ],

    y: [
        0x03, 0xca, 0x0c, 0xfe, 0x4b, 0x3b, 0xc6, 0xdd, 0xf3, 0x46, 0xd4, 0x9d, 0x06, 0xea, 0x0e,
        0xd3, 0x4e, 0x62, 0x10, 0x62, 0xc0, 0xe0, 0x56, 0xc1, 0xd0, 0x40, 0x5d, 0x26, 0x6e, 0x10,
        0x26, 0x8a,
    ],
};

const PEDERSEN_POINTS: [ECPoint; 4] = [
    ECPoint {
        x: [
            0x02, 0x34, 0x28, 0x7d, 0xcb, 0xaf, 0xfe, 0x7f, 0x96, 0x9c, 0x74, 0x86, 0x55, 0xfc,
            0xa9, 0xe5, 0x8f, 0xa8, 0x12, 0x0b, 0x6d, 0x56, 0xeb, 0x0c, 0x10, 0x80, 0xd1, 0x79,
            0x57, 0xeb, 0xe4, 0x7b,
        ],

        y: [
            0x03, 0xb0, 0x56, 0xf1, 0x00, 0xf9, 0x6f, 0xb2, 0x1e, 0x88, 0x95, 0x27, 0xd4, 0x1f,
            0x4e, 0x39, 0x94, 0x01, 0x35, 0xdd, 0x7a, 0x6c, 0x94, 0xcc, 0x6e, 0xd0, 0x26, 0x8e,
            0xe8, 0x9e, 0x56, 0x15,
        ],
    },
    ECPoint {
        x: [
            0x04, 0xfa, 0x56, 0xf3, 0x76, 0xc8, 0x3d, 0xb3, 0x3f, 0x9d, 0xab, 0x26, 0x56, 0x55,
            0x8f, 0x33, 0x99, 0x09, 0x9e, 0xc1, 0xde, 0x5e, 0x30, 0x18, 0xb7, 0xa6, 0x93, 0x2d,
            0xba, 0x8a, 0xa3, 0x78,
        ],

        y: [
            0x03, 0xfa, 0x09, 0x84, 0xc9, 0x31, 0xc9, 0xe3, 0x81, 0x13, 0xe0, 0xc0, 0xe4, 0x7e,
            0x44, 0x01, 0x56, 0x27, 0x61, 0xf9, 0x2a, 0x7a, 0x23, 0xb4, 0x51, 0x68, 0xf4, 0xe8,
            0x0f, 0xf5, 0xb5, 0x4d,
        ],
    },
    ECPoint {
        x: [
            0x04, 0xba, 0x4c, 0xc1, 0x66, 0xbe, 0x8d, 0xec, 0x76, 0x49, 0x10, 0xf7, 0x5b, 0x45,
            0xf7, 0x4b, 0x40, 0xc6, 0x90, 0xc7, 0x47, 0x09, 0xe9, 0x0f, 0x3a, 0xa3, 0x72, 0xf0,
            0xbd, 0x2d, 0x69, 0x97,
        ],

        y: [
            0x00, 0x40, 0x30, 0x1c, 0xf5, 0xc1, 0x75, 0x1f, 0x4b, 0x97, 0x1e, 0x46, 0xc4, 0xed,
            0xe8, 0x5f, 0xca, 0xc5, 0xc5, 0x9a, 0x5c, 0xe5, 0xae, 0x7c, 0x48, 0x15, 0x1f, 0x27,
            0xb2, 0x4b, 0x21, 0x9c,
        ],
    },
    ECPoint {
        x: [
            0x05, 0x43, 0x02, 0xdc, 0xb0, 0xe6, 0xcc, 0x1c, 0x6e, 0x44, 0xcc, 0xa8, 0xf6, 0x1a,
            0x63, 0xbb, 0x2c, 0xa6, 0x50, 0x48, 0xd5, 0x3f, 0xb3, 0x25, 0xd3, 0x6f, 0xf1, 0x2c,
            0x49, 0xa5, 0x82, 0x02,
        ],

        y: [
            0x01, 0xb7, 0x7b, 0x3e, 0x37, 0xd1, 0x35, 0x04, 0xb3, 0x48, 0x04, 0x62, 0x68, 0xd8,
            0xae, 0x25, 0xce, 0x98, 0xad, 0x78, 0x3c, 0x25, 0x56, 0x1a, 0x87, 0x9d, 0xcc, 0x77,
            0xe9, 0x9c, 0x24, 0x26,
        ],
    },
];

pub fn pedersen_hash(a: &mut FieldElement, b: &FieldElement) {
    if !(*a >= FieldElement::ZERO && *a < P) {
        panic!("a is not in the field");
    }

    if !(*b >= FieldElement::ZERO && *b < P) {
        panic!("b is not in the field");
    }

    unsafe {
        cx_bn_lock(32, 0);
    }

    /* shift point */
    let mut sp: cx_ecpoint_t = Default::default();

    let sp_x = PEDERSEN_SHIFT.x;
    let sp_y = PEDERSEN_SHIFT.y;
    unsafe {
        cx_ecpoint_alloc(&mut sp as *mut cx_ecpoint_t, CX_CURVE_Stark256);
        cx_ecpoint_init(
            &mut sp as *mut cx_ecpoint_t,
            sp_x.as_ptr(),
            32,
            sp_y.as_ptr(),
            32,
        );
    }

    let (a0, a1) = a.value.as_slice().split_at(1);
    let (b0, b1) = b.value.as_slice().split_at(1);

    double_accum_ec_mul(&mut sp, a1, 31, b1, 31, 0);
    double_accum_ec_mul(&mut sp, a0, 1, b0, 1, 1);

    let mut tmp: [u8; 32] = [0; 32];
    unsafe {
        cx_ecpoint_export(
            &sp as *const cx_ecpoint_t,
            a.value.as_mut_ptr(),
            32,
            tmp.as_mut_ptr(),
            32,
        );
        cx_ecpoint_destroy(&mut sp as *mut cx_ecpoint_t);
        cx_bn_unlock();
    }
}

fn double_accum_ec_mul(
    h: &mut cx_ecpoint_t,
    buf1: &[u8],
    len1: usize,
    buf2: &[u8],
    len2: usize,
    idx: usize,
) {
    let px = PEDERSEN_POINTS[idx].x;
    let py = PEDERSEN_POINTS[idx].y;
    let mut p: cx_ecpoint_t = Default::default();

    unsafe {
        cx_ecpoint_alloc(&mut p as *mut cx_ecpoint_t, CX_CURVE_Stark256);
        cx_ecpoint_init(
            &mut p as *mut cx_ecpoint_t,
            px.as_ptr(),
            32,
            py.as_ptr(),
            32,
        );
    }

    let qx = PEDERSEN_POINTS[idx + 2].x;
    let qy = PEDERSEN_POINTS[idx + 2].y;
    let mut q: cx_ecpoint_t = Default::default();

    unsafe {
        cx_ecpoint_alloc(&mut q as *mut cx_ecpoint_t, CX_CURVE_Stark256);
        cx_ecpoint_init(
            &mut q as *mut cx_ecpoint_t,
            qx.as_ptr(),
            32,
            qy.as_ptr(),
            32,
        );
    }

    let mut pad1: [u8; 32] = [0; 32];
    let mut pad2: [u8; 32] = [0; 32];

    let allzero1 = buf1.iter().all(|&x| x == 0);
    let allzero2 = buf2.iter().all(|&x| x == 0);

    if !allzero1 && !allzero2 {
        pad1[(32 - len1)..].copy_from_slice(&buf1[..len1]);
        pad2[(32 - len2)..].copy_from_slice(&buf2[..len2]);
        let mut r: cx_ecpoint_t = Default::default();
        unsafe {
            cx_ecpoint_alloc(&mut r as *mut cx_ecpoint_t, CX_CURVE_Stark256);
            cx_ecpoint_double_scalarmul(
                &mut r as *mut cx_ecpoint_t,
                &mut p as *mut cx_ecpoint_t,
                &mut q as *mut cx_ecpoint_t,
                pad1.as_ptr(),
                32,
                pad2.as_ptr(),
                32,
            );
            cx_ecpoint_add(
                h as *mut cx_ecpoint_t,
                h as *const cx_ecpoint_t,
                &r as *const cx_ecpoint_t,
            );
            cx_ecpoint_destroy(&mut r as *mut cx_ecpoint_t);
        }
    } else if !allzero1 {
        pad1[32 - len1..].copy_from_slice(&buf1[..len1]);
        unsafe {
            cx_ecpoint_rnd_scalarmul(&mut p as *mut cx_ecpoint_t, pad1.as_ptr(), 32);
            cx_ecpoint_add(
                h as *mut cx_ecpoint_t,
                h as *const cx_ecpoint_t,
                &p as *const cx_ecpoint_t,
            );
        }
    } else if !allzero2 {
        pad2[32 - len2..].copy_from_slice(&buf2[..len2]);
        unsafe {
            cx_ecpoint_rnd_scalarmul(&mut q as *mut cx_ecpoint_t, pad2.as_ptr(), 32);
            cx_ecpoint_add(
                h as *mut cx_ecpoint_t,
                h as *const cx_ecpoint_t,
                &q as *const cx_ecpoint_t,
            );
        }
    }
    unsafe {
        cx_ecpoint_destroy(&mut p as *mut cx_ecpoint_t);
        cx_ecpoint_destroy(&mut q as *mut cx_ecpoint_t);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PedersenHasher {
    state: FieldElement,
    nb_fe: usize,
}

use super::HasherTrait;

impl HasherTrait for PedersenHasher {
    /// Absorbs message into the hash.
    fn update(&mut self, msg: FieldElement) {
        pedersen_hash(&mut self.state, &msg);
        self.nb_fe += 1;
    }

    fn finalize(self) -> FieldElement {
        self.state
    }
}

impl PedersenHasher {
    pub fn get_nb_fe(&self) -> usize {
        self.nb_fe
    }
}
