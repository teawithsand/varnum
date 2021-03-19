use core::cmp::Ordering;

/// Compares two numbers returning `core::cmp::Ordering` enum.
///
/// # Return
/// Returns `Equal` variant when both lhs and rhs are zero sized.
pub fn compare_unsigned(lhs: &[u8], rhs: &[u8]) -> core::cmp::Ordering {
    let mut lhs_idx = 0;
    let mut rhs_idx = 0;
    // 1. Strip leading zeros
    while lhs_idx < lhs.len() && lhs[lhs_idx] == 0 {
        lhs_idx += 1;
    }
    while rhs_idx < rhs.len() && rhs[rhs_idx] == 0 {
        rhs_idx += 1;
    }

    // 2. Do size comparing
    match (lhs.len() - lhs_idx).cmp(&(rhs.len() - rhs_idx)) {
        Ordering::Equal => {} // pass here; do actual compare
        o => {
            return o;
        }
    }
    for (l, r) in lhs[lhs_idx..]
        .iter()
        .copied()
        .zip(rhs[rhs_idx..].iter().copied())
    {
        match l.cmp(&r) {
            Ordering::Equal => {}
            o => {
                return o;
            }
        }
    }

    // they are equal
    Ordering::Equal
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compare_eq() {
        for (l, r) in [
            (&[][..], &[][..]),
            (&[1, 2, 3][..], &[1, 2, 3][..]),
            (&[0, 0, 0, 0, 1, 2, 3][..], &[1, 2, 3][..]),
            (&[1, 2, 3][..], &[0, 0, 0, 1, 2, 3][..]),
        ]
        .iter()
        {
            let v = compare_unsigned(&l[..], &r[..]);
            assert_eq!(v, Ordering::Equal, "{:?} == {:?}", &l[..], &r[..]);
        }
    }
}
