use crate::num::{BigNum, BigNumMut, DynamicBigNum, UnsignedNumDigit};
use crate::ops::UnsignedEngine;

fn mul_accumulate_digit<D>(
    res: &mut impl BigNumMut<Digit = D>,
    lhs: &impl BigNum<Digit = D>,
    d: D,
    offset: usize,
) -> D
where
    D: UnsignedNumDigit,
{
    assert!(res.len() >= lhs.len());
    let mut carry = D::ZERO;

    let mut i = offset;
    let mut j = offset;
    while i < lhs.len() {
        let l = lhs[i];

        let (n, m) = l.mul_to_parts(d);
        debug_assert!(n < D::MAX);

        let mut new_carry = D::ZERO;
        let (z, overflow) = res[j].overflowing_add(m);
        if overflow {
            new_carry += D::ONE;
        }

        let (z, overflow) = z.overflowing_add(carry);
        if overflow {
            new_carry += D::ONE;
        }

        res[j] = z;
        new_carry += n;
        carry = new_carry;

        j += 1;
        i += 1;
    }

    while j < res.len() && carry != D::ZERO {
        let (n, overflow) = res[j].overflowing_add(carry);
        carry = if overflow { D::ONE } else { D::ZERO };
        res[j] = n;
        j += 1;
    }

    carry
}

/// Simple engine performs all operations in simplest possible way without any allocations on heap.
pub struct SimpleEngine {}
impl UnsignedEngine for SimpleEngine {
    fn add_accumulate<D>(res: &mut impl BigNumMut<Digit = D>, rhs: &impl BigNum<Digit = D>) -> bool
    where
        D: UnsignedNumDigit,
    {
        assert!(res.len() >= rhs.len());
        let mut i = 0;
        let mut j = 0;

        let mut carry = false;
        while i < res.len() && j < rhs.len() {
            let l = res[i];
            let r = rhs[j];

            let (n, overflow) = l.overflowing_add(r);
            let (n, carry_overflow) = n.overflowing_add(if carry { D::ONE } else { D::ZERO });

            debug_assert!(!(carry_overflow && overflow));
            carry = overflow || carry_overflow;

            res[i] = n;

            i += 1;
            j += 1;
        }

        while i < res.len() && carry {
            let (n, overflow) = res[i].overflowing_add(if carry { D::ONE } else { D::ZERO });
            carry = overflow;

            res[i] = n;
            i += 1;
        }
        /*

        wile j < rhs.len() && !carry {
            carry = rhs[j] != D::ZERO;
            j += 1;
        }
        */

        carry
    }

    fn sub_accumulate<D>(res: &mut impl BigNumMut<Digit = D>, rhs: &impl BigNum<Digit = D>) -> bool
    where
        D: UnsignedNumDigit,
    {
        assert!(res.len() >= rhs.len());

        let mut i = 0;
        let mut j = 0;

        let mut borrow = false;
        while i < res.len() && j < rhs.len() {
            let l = res[i];
            let r = rhs[j];

            let (n, borrow_overflow) = l.overflowing_sub(if borrow { D::ONE } else { D::ZERO });
            let (n, overflow) = n.overflowing_sub(r);
            borrow = overflow || borrow_overflow;

            res[i] = n;

            i += 1;
            j += 1;
        }

        // note: only one of blocks below will be executed
        // as at least one of conditions i == res.len() or j == rhs.len() is met
        // note #2: with assert above only j == rhs.len() can be met

        /*
        let mut fits = true;
        while j < rhs.len() && fits {
            fits = rhs[j] == D::ZERO;
            j += 1;
        }
        */

        while i < res.len() && borrow {
            let (n, overflow) = res[i].overflowing_sub(if borrow { D::ONE } else { D::ZERO });
            borrow = overflow;
            res[i] = n;
            i += 1;
        }

        borrow
    }

    fn shift_left_u32<D>(res: &mut impl BigNumMut<Digit = D>, n: u32) -> bool
    where
        D: UnsignedNumDigit,
    {
        let ov_bytes = (n / D::NUM_BITS) as usize;

        let n = n % (res.len() as u32 * D::NUM_BITS);
        let bytes = (n / D::NUM_BITS) as usize;
        let bits = (n % D::NUM_BITS) as usize;

        println!("{}b {}B", bits, bytes);

        // 1. Shift bytes
        // well, shifting left in little endian is shifting right actually...
        for i in (bytes..res.len()).rev() {
            res[i] = res[i - bytes];
        }

        // 2. Fill rest with zeros
        for i in 0..bytes {
            res[i] = D::ZERO;
        }

        // 2. Shift bits
        // however each digit may have it's own endianness so use bitwise ops which do actual left shift
        // on each number if it's required
        if bits > 0 {
            // Mask used to get lowest bits from each digit to put them as highest bits of other digit
            let shift_mask = (!D::ZERO) << (D::NUM_BITS - bits as u32);

            for i in (0..res.len()).rev() {
                if i != res.len() - 1 {                     // on the rightmost digit bits are lost, there is no number to transfer them to
                    let imm = (res[i] & shift_mask) >> (D::NUM_BITS - bits as u32);
                    res[i + 1] |= imm;
                }
                res[i] = res[i] << bits as u32;
            }
        }


        ov_bytes >= res.len()
    }

    fn shift_right_u32<D>(res: &mut impl BigNumMut<Digit = D>, n: u32) -> bool
    where
        D: UnsignedNumDigit,
    {
        unimplemented!();
    }

    fn mul_accumulate<D>(
        res: &mut impl BigNumMut<Digit = D>,
        lhs: &impl BigNum<Digit = D>,
        rhs: &impl BigNum<Digit = D>,
    ) -> bool
    where
        D: UnsignedNumDigit,
    {
        // only simple multiplication algortihm
        let mut overflow = false;
        for i in 0..rhs.len() {
            overflow |= mul_accumulate_digit(res, lhs, rhs[i], i) != D::ZERO;
        }
        overflow
    }

    fn add_resize<D, M>(res: &mut M, rhs: &impl BigNum<Digit = D>)
    where
        M: DynamicBigNum<Digit = D> + BigNumMut<Digit = D>,
        D: UnsignedNumDigit,
    {
        if res.len() < rhs.len() {
            res.resize(rhs.len());
        }

        let carry = Self::add_accumulate(res, rhs);
        if carry {
            res.resize(res.len() + 1);
            let res_sz = res.len();

            let dst = &mut res[res_sz - 1];
            debug_assert!(*dst == D::ZERO);
            *dst = D::ONE;
        }
    }

    fn sub_resize<D, M>(res: &mut M, rhs: &impl BigNum<Digit = D>) -> bool
    where
        M: DynamicBigNum<Digit = D> + BigNumMut<Digit = D>,
        D: UnsignedNumDigit,
    {
        if res.len() < rhs.len() {
            res.resize(rhs.len());
        }

        // note: in sub carry can't be handled as it potentially can set infinite 0xff bytes in order to express minus number
        // so here it's just returned, so caller knows that rhs > res

        Self::sub_accumulate(res, rhs)
    }

    fn mul_resize<D, M>(res: &mut M, lhs: &impl BigNum<Digit = D>, rhs: &impl BigNum<Digit = D>)
    where
        M: DynamicBigNum<Digit = D> + BigNumMut<Digit = D>,
        D: UnsignedNumDigit,
    {
        let mut lhs_real_size = lhs.len();
        for i in (0..lhs.len()).rev() {
            if lhs[i] != D::ZERO {
                break;
            }
            lhs_real_size -= 1;
        }

        let mut rhs_real_size = rhs.len();
        for i in (0..rhs.len()).rev() {
            if rhs[i] != D::ZERO {
                break;
            }
            rhs_real_size -= 1;
        }

        if res.len() < lhs_real_size + rhs_real_size {
            res.resize(lhs_real_size + rhs_real_size);
        }

        let overflow = Self::mul_accumulate(res, lhs, rhs);
        debug_assert!(!overflow);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::num::{BigNum, DefaultBigNumDigit, VecBigNum};

    #[test]
    fn test_add_accumulate_u16() {
        for a in 0..(std::u16::MAX) {
            for b in 0..(std::u16::MAX >> 8) {
                let bn_a = VecBigNum::from(Vec::from(&a.to_le_bytes()[..]));
                let bn_b = VecBigNum::from(Vec::from(&b.to_le_bytes()[..]));
                let (c, overflow) = a.overflowing_add(b);

                let mut res: VecBigNum<u8> =
                    VecBigNum::new_zeroed_sized(core::mem::size_of::<u16>());
                let ov1 = SimpleEngine::add_accumulate(&mut res, &bn_a);
                let ov2 = SimpleEngine::add_accumulate(&mut res, &bn_b);

                assert!(!ov1);
                assert_eq!(overflow, ov2);

                assert_eq!(&res.into_inner()[..], &c.to_le_bytes()[..]);
            }
        }
    }

    #[test]
    fn test_sub_accumulate_u16() {
        for a in 0..(std::u16::MAX) {
            for b in 0..(std::u16::MAX >> 8) {
                let bn_a = VecBigNum::from(Vec::from(&a.to_le_bytes()[..]));
                let bn_b = VecBigNum::from(Vec::from(&b.to_le_bytes()[..]));
                let (c, overflow) = a.overflowing_sub(b);

                let mut res: VecBigNum<u8> =
                    VecBigNum::new_zeroed_sized(core::mem::size_of::<u16>());
                let ov1 = SimpleEngine::add_accumulate(&mut res, &bn_a);
                let ov2 = SimpleEngine::sub_accumulate(&mut res, &bn_b);

                assert!(!ov1);
                assert_eq!(overflow, ov2);

                assert_eq!(&res.into_inner()[..], &c.to_le_bytes()[..]);
            }
        }
    }

    #[test]
    fn test_mul_accumulate_u16() {
        for a in 0..(std::u16::MAX) {
            for b in 0..(std::u16::MAX >> 8) {
                let bn_a = VecBigNum::from(Vec::from(&a.to_le_bytes()[..]));
                let bn_b = VecBigNum::from(Vec::from(&b.to_le_bytes()[..]));
                let (c, overflow) = a.overflowing_mul(b);

                let mut res: VecBigNum<u8> =
                    VecBigNum::new_zeroed_sized(core::mem::size_of::<u16>());
                let ov2 = SimpleEngine::mul_accumulate(&mut res, &bn_a, &bn_b);

                assert_eq!(overflow, ov2);
                assert_eq!(&res.into_inner()[..], &c.to_le_bytes()[..]);
            }
        }
    }

    #[test]
    fn test_shift_left_u32() {
        for a in 0..(std::u16::MAX) {
            for b in 0..18u32 {
                let bn_a = VecBigNum::from(Vec::from(&a.to_le_bytes()[..]));
                let (c, overflow) = a.overflowing_shl(b);

                let mut res: VecBigNum<u8> =
                    VecBigNum::new_zeroed_sized(core::mem::size_of::<u16>());
                SimpleEngine::add_accumulate(&mut res, &bn_a);
                let ov2 = SimpleEngine::shift_left_u32(&mut res, b);

                assert_eq!(overflow, ov2);
                assert_eq!(&res.into_inner()[..], &c.to_le_bytes()[..]);
            }
        }
    }
}
