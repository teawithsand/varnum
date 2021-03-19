use crate::iutil::{assert_dst_op, assert_in_place_op};

/// Subtracts two numbers.
///
/// # Return
/// Returns true if borrowing from not existing bit has been performed.
pub fn sub_unsigned_in_place(res: &mut [u8], rhs: &[u8]) -> bool {
    assert_in_place_op(res, rhs);

    let mut burrow = false;
    let mut res_idx = res.len() - 1;
    let mut rhs_idx = rhs.len() - 1;
    loop {
        let l = res[res_idx];
        let r = rhs[rhs_idx];

        let (n, overflow) = l.overflowing_sub(r);
        if overflow {
            if res_idx == 0 {
                burrow = true;
            } else {
                let mut found = false;
                for j in (0..=(res_idx - 1)).rev() {
                    if res[j] > 0 {
                        res[j] -= 1;
                        found = true;
                        break;
                    } else {
                        res[j] = 0xff;
                    }
                }
                if !found {
                    burrow = true;
                }
            }
        }

        res[res_idx] = n;
        if rhs_idx == 0 || res_idx == 0 {
            break;
        }
        res_idx -= 1;
        rhs_idx -= 1;
    }

    burrow
}

/// Subtracts two unsigned numbers from each other and puts result in res.
pub fn sub_unsigned(res: &mut [u8], lhs: &[u8], rhs: &[u8]) -> bool {
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
    let burrow = sub_unsigned_in_place(res, rhs);

    // behaviour here could be: assume res is lhs bytes integer
    // or res is it's own bytes integer
    // here res is it's own bytes integers, as zeroing code is commented out
    /*
    // 4. Zero out all bits in res that are size difference between lhs and res
    for i in 0..delta_sz_a {
        res[i] = 0;
    }
    */

    burrow
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

                let burrow = sub_unsigned_in_place(&mut lb[..], &rb[..]);

                let (res, overflow) = l.overflowing_sub(r);
                assert_eq!(&res.to_be_bytes()[..], &lb[..]);
                assert_eq!(overflow, burrow);
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

                let burrow = sub_unsigned(&mut res, &lb[..], &rb[..]);

                let (actual_res, overflow) = l.overflowing_sub(r);

                assert_eq!(&actual_res.to_be_bytes()[..], &res[..]);
                assert_eq!(overflow, burrow);
            }
        }
    }
}
