use crate::num::UnsignedNumDigit;

pub fn assert_in_place_op<D>(res: &mut [D], rhs: &[D])
where
    D: UnsignedNumDigit,
{
    assert!(res.len() > 0, "Can't operate on zero bytes result");
    assert!(
        res.len() >= rhs.len(),
        "Res must be bigger than or equal in size to rhs"
    );
}
pub fn assert_dst_op(res: &mut [u8], lhs: &[u8], rhs: &[u8]) {
    assert!(res.len() > 0, "Can't operate on zero bytes result");
    assert!(
        res.len() >= rhs.len(),
        "Res must be bigger than or equal in size to rhs"
    );
    assert!(
        res.len() >= lhs.len(),
        "Res must be bigger than or equal in size to lhs"
    );
}
