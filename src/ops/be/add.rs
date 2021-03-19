use crate::iutil::{assert_in_place_op, assert_dst_op};
use crate::num::UnsignedNumDigit;

/// Adds two unsigned numbers.
/// # Panic
/// Panics when res.len() < rhs.len().
///
/// # Return
/// Returned value is `true` when overflow occurred.
pub fn add_unsigned_in_place<D>(res: &mut [D], rhs: &[D]) -> bool
    where D: UnsignedNumDigit
{
    assert_in_place_op(res, rhs);
    let mut carry = false;

    let mut res_idx = res.len() - 1;
    let mut rhs_idx = rhs.len() - 1;
    loop {
        let l = res[res_idx];
        let r = rhs[rhs_idx];

        let (mut n, mut overflow) = l.overflowing_add(r);
        if overflow {
            if carry {
                n += D::ONE;
            }
            carry = true;
        } else if carry {
            let t = n.overflowing_add(D::ONE);
            n = t.0;
            overflow = t.1;

            if overflow {
                carry = true;
            } else {
                carry = false;
            }
        }

        res[res_idx] = n;

        if res_idx == 0 || rhs_idx == 0 {
            break;
        }

        res_idx -= 1;
        rhs_idx -= 1;
    }

    if carry && res_idx > 0 {
        res_idx -= 1;
        res[res_idx] = D::ONE;
        carry = false;
    }

    carry
}

/// Adds two unsigned numbers.
///
/// # Panic
/// Panics when res.len() < rhs.len().
///
/// # Return
/// Returned value is `true` when overflow occurred.
pub fn add_unsigned(res: &mut [u8], lhs: &[u8], rhs: &[u8]) -> bool {
    assert_dst_op(res, lhs, rhs);

    let delta_sz_a = res.len() - lhs.len();

    // 1. Put a into res
    for i in 0..(res.len().min(lhs.len())) {
        res[delta_sz_a + i] = lhs[i];
    }
    // 2. Zero out rest of res
    for i in 0..delta_sz_a {
        res[i] = 0;
    }

    // 3. Run unsigned add; Carry will be false if it could be stored in res already
    add_unsigned_in_place(res, rhs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_all_u16_nums() {
        for l in 0..std::u16::MAX {
            for r in 0..(std::u16::MAX >> 8) {
                let mut lb = l.to_be_bytes();
                let rb = r.to_be_bytes();

                let carry = add_unsigned_in_place(&mut lb[..], &rb[..]);

                let (res, overflow) = l.overflowing_add(r);
                assert_eq!(&res.to_be_bytes()[..], &lb[..]);
                assert_eq!(overflow, carry);
            }
        }
    }

    #[test]
    fn test_all_u16_nums_into_u32() {
        for l in 0..std::u16::MAX {
            for r in 0..(std::u16::MAX >> 8) {
                let lb = l.to_be_bytes();
                let rb = r.to_be_bytes();

                let l = l as u32;
                let r = r as u32;

                let mut res = [0u8; 4];
                let bytes_res = (l + r).to_be_bytes();
                let c = add_unsigned(&mut res, &lb[..], &rb[..]);
                assert_eq!(&bytes_res[..], &res[..]);
                assert!(!c);
            }
        }
    }
}
