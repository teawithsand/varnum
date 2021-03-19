use core::convert::TryFrom;
use core::hash::Hash;
use core::ops::*;

// TODO(teaiwthsand): add things like wrapping add/sub/mul/div and others to these types
// TODO(teawithsand): add outpu=self for all ops

/// Type of single digit used in bignum operations.
/// Implemented by all primitive types.
/// Endianness should be handled internally.
pub trait UnsignedNumDigit:
    Sized
    + core::fmt::Debug 
    + Copy
    + Clone
    + Add<Output=Self>
    + AddAssign
    + Sub<Output=Self>
    + SubAssign
    + Mul<Output=Self>
    + MulAssign
    + Div<Output=Self>
    + DivAssign
    + Rem<Output=Self>
    + RemAssign
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Shl
    + ShlAssign
    + Shr
    + ShrAssign
    + Not
    + PartialEq
    + Eq
    + Hash
    + PartialOrd
    + Ord
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<u128>
    + TryFrom<i8>
    + TryFrom<i16>
    + TryFrom<i32>
    + TryFrom<i64>
    + TryFrom<i128>
    + Default
{
    /// Signed version of self.
    type Signed: SignedNumDigit;

    /// Type of exponent used for pow.
    type Exponent: UnsignedNumDigit;

    /// Number that is twice as big as current one.
    // type Double: UnsignedNumDigit;

    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    const MIN: Self;
    const NUM_BITS: u32;

    fn pow(self, other: Self::Exponent) -> Self;
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
    fn wrapping_div(self, other: Self) -> Self;
    fn wrapping_mul(self, other: Self) -> Self;
    fn wrapping_pow(self, other: Self::Exponent) -> Self;
    fn wrapping_rem(self, other: Self) -> Self;
    fn wrapping_shl(self, other: Self::Exponent) -> Self;
    fn wrapping_shr(self, other: Self::Exponent) -> Self;
    fn wrapping_div_euclid(self, other: Self) -> Self;
    fn wrapping_rem_euclid(self, other: Self) -> Self;

    fn checked_add(self, other: Self) -> Option<Self>;
    fn checked_sub(self, other: Self) -> Option<Self>;
    fn checked_div(self, other: Self) -> Option<Self>;
    fn checked_mul(self, other: Self) -> Option<Self>;
    fn checked_pow(self, other: Self::Exponent) -> Option<Self>;
    fn checked_rem(self, other: Self) -> Option<Self>;
    fn checked_shl(self, other: Self::Exponent) -> Option<Self>;
    fn checked_shr(self, other: Self::Exponent) -> Option<Self>;
    /*
    fn unchecked_add(self, other: Self) -> Option<Self>;
    fn unchecked_mul(self, other: Self) -> Option<Self>;
    fn unchecked_sub(self, other: Self) -> Option<Self>;
    */
    fn checked_div_euclid(self, other: Self) -> Option<Self>;
    fn checked_rem_euclid(self, other: Self) -> Option<Self>;

    fn overflowing_add(self, other: Self) -> (Self, bool);
    fn overflowing_sub(self, other: Self) -> (Self, bool);
    fn overflowing_div(self, other: Self) -> (Self, bool);
    fn overflowing_mul(self, other: Self) -> (Self, bool);
    fn overflowing_pow(self, other: Self::Exponent) -> (Self, bool);
    fn overflowing_rem(self, other: Self) -> (Self, bool);
    fn overflowing_shl(self, other: Self::Exponent) -> (Self, bool);
    fn overflowing_shr(self, other: Self::Exponent) -> (Self, bool);
    fn overflowing_div_euclid(self, other: Self) -> (Self, bool);
    fn overflowing_rem_euclid(self, other: Self) -> (Self, bool);



    //// Multiples two numbes, so that higher NUM_BITS are in 1st self and
    //// lower NUM_BITS are in 2nd self
    fn mul_to_parts(self, other: Self) -> (Self, Self);
}

/// Type of single digit used in bignum operations.
/// Implemented by all primitive types.
/// Endianness should be handled internally.
pub trait SignedNumDigit:
    Sized
    + Copy
    + Clone
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Rem
    + RemAssign
    + Neg
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Shl
    + ShlAssign
    + Shr
    + ShrAssign
    + Not
    + PartialEq
    + Eq
    + Hash
    + PartialOrd
    + Ord
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<u128>
    + TryFrom<i8>
    + TryFrom<i16>
    + TryFrom<i32>
    + TryFrom<i64>
    + TryFrom<i128>
    + Default
{
    /// Signed version of self.
    type Unsigned: UnsignedNumDigit;

    /// Type of exponent used for pow.
    type Exponent: UnsignedNumDigit;

    /// Number that is twice as big as current one.
    // type Double: SignedNumDigit;

    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    const MIN: Self;
    const NUM_BITS: u32;

    fn abs(self) -> Self;
    fn pow(self, other: Self::Exponent) -> Self;
    fn wrapping_abs(self) -> Self;
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
    fn wrapping_div(self, other: Self) -> Self;
    fn wrapping_mul(self, other: Self) -> Self;
    fn wrapping_neg(self) -> Self;
    fn wrapping_pow(self, other: Self::Exponent) -> Self;
    fn wrapping_rem(self, other: Self) -> Self;
    fn wrapping_shl(self, other: Self::Exponent) -> Self;
    fn wrapping_shr(self, other: Self::Exponent) -> Self;
    fn wrapping_div_euclid(self, other: Self) -> Self;
    fn wrapping_rem_euclid(self, other: Self) -> Self;

    fn checked_abs(self) -> Option<Self>;
    fn checked_add(self, other: Self) -> Option<Self>;
    fn checked_sub(self, other: Self) -> Option<Self>;
    fn checked_div(self, other: Self) -> Option<Self>;
    fn checked_mul(self, other: Self) -> Option<Self>;
    fn checked_neg(self) -> Option<Self>;
    fn checked_pow(self, other: Self::Exponent) -> Option<Self>;
    fn checked_rem(self, other: Self) -> Option<Self>;
    fn checked_shl(self, other: Self::Exponent) -> Option<Self>;
    fn checked_shr(self, other: Self::Exponent) -> Option<Self>;
    /*
    fn unchecked_add(self, other: Self) -> Option<Self>;
    fn unchecked_mul(self, other: Self) -> Option<Self>;
    fn unchecked_sub(self, other: Self) -> Option<Self>;
    */
    fn checked_div_euclid(self, other: Self) -> Option<Self>;
    fn checked_rem_euclid(self, other: Self) -> Option<Self>;

    fn overflowing_abs(self) -> (Self, bool);
    fn overflowing_add(self, other: Self) -> (Self, bool);
    fn overflowing_sub(self, other: Self) -> (Self, bool);
    fn overflowing_div(self, other: Self) -> (Self, bool);
    fn overflowing_mul(self, other: Self) -> (Self, bool);
    fn overflowing_neg(self) -> (Self, bool);
    fn overflowing_pow(self, other: Self::Exponent) -> (Self, bool);
    fn overflowing_rem(self, other: Self) -> (Self, bool);
    fn overflowing_shl(self, other: Self::Exponent) -> (Self, bool);
    fn overflowing_shr(self, other: Self::Exponent) -> (Self, bool);
    fn overflowing_div_euclid(self, other: Self) -> (Self, bool);
    fn overflowing_rem_euclid(self, other: Self) -> (Self, bool);
}

macro_rules! generate_mapping {
    {
        $(
            $vis:vis fn $name:ident($($arg_name:ident: $arg_ty:ty),*) -> $ret:ty;
        )*
    } => {
        $(
            #[inline]
            fn $name(self, $($arg_name: $arg_ty),*) -> $ret {
                self.$name($($arg_name),*)
            }
        )*
    };
}

macro_rules! derive_signed_num_digit {
    ($type:ident, $unsigned:ident, $double:ty) => {
        impl SignedNumDigit for $type {
            type Unsigned = $unsigned;
            type Exponent = u32;
            // type Double = $double;

            const ZERO: Self = 0 as Self;
            const ONE: Self = 1 as Self;
            const MAX: Self = (!(0 as Self::Unsigned) >> 1) as Self;
            const MIN: Self = -(Self::MAX) - 1;
            const NUM_BITS: u32 = (core::mem::size_of::<$type>() as u32) * 8;

            generate_mapping! {
                fn abs() -> Self;
                fn pow(exp: Self::Exponent) -> Self;
                fn wrapping_abs() -> Self;
                fn wrapping_add(other: Self) -> Self;
                fn wrapping_sub(other: Self) -> Self;
                fn wrapping_div(other: Self) -> Self;
                fn wrapping_mul(other: Self) -> Self;
                fn wrapping_neg() -> Self;
                fn wrapping_pow(other: Self::Exponent) -> Self;
                fn wrapping_rem(other: Self) -> Self;
                fn wrapping_shl(other: u32) -> Self;
                fn wrapping_shr(other: u32) -> Self;
                fn wrapping_div_euclid(other: Self) -> Self;
                fn wrapping_rem_euclid(other: Self) -> Self;

                fn checked_abs() -> Option<Self>;
                fn checked_add(other: Self) -> Option<Self>;
                fn checked_sub(other: Self) -> Option<Self>;
                fn checked_div(other: Self) -> Option<Self>;
                fn checked_mul(other: Self) -> Option<Self>;
                fn checked_neg() -> Option<Self>;
                fn checked_pow(other: Self::Exponent) -> Option<Self>;
                fn checked_rem(other: Self) -> Option<Self>;
                fn checked_shl(other: Self::Exponent) -> Option<Self>;
                fn checked_shr(other: Self::Exponent) -> Option<Self>;
                /*
                fn unchecked_add(other: Self) -> Option<Self>;
                fn unchecked_mul(other: Self) -> Option<Self>;
                fn unchecked_sub(other: Self) -> Option<Self>;
                */
                fn checked_div_euclid(other: Self) -> Option<Self>;
                fn checked_rem_euclid(other: Self) -> Option<Self>;

                fn overflowing_abs() -> (Self, bool);
                fn overflowing_add(other: Self) -> (Self, bool);
                fn overflowing_sub(other: Self) -> (Self, bool);
                fn overflowing_div(other: Self) -> (Self, bool);
                fn overflowing_mul(other: Self) -> (Self, bool);
                fn overflowing_neg() -> (Self, bool);
                fn overflowing_pow(other: Self::Exponent) -> (Self, bool);
                fn overflowing_rem(other: Self) -> (Self, bool);
                fn overflowing_shl(other: Self::Exponent) -> (Self, bool);
                fn overflowing_shr(other: Self::Exponent) -> (Self, bool);
                fn overflowing_div_euclid(other: Self) -> (Self, bool);
                fn overflowing_rem_euclid(other: Self) -> (Self, bool);
            }
        }
    };
}

macro_rules! derive_unsigned_num_digit {
    ($type:ident, $signed:ident, $double:ty) => {
        impl UnsignedNumDigit for $type {
            type Signed = $signed;
            type Exponent = u32;
            // type Double = $double;

            const ZERO: Self = 0 as Self;
            const ONE: Self = 1 as Self;
            const MAX: Self = !(0 as Self);
            const MIN: Self = 0 as Self;
            const NUM_BITS: u32 = (core::mem::size_of::<$type>() as u32) * 8;

            generate_mapping! {
                fn pow(exp: Self::Exponent) -> Self;
                fn wrapping_add(other: Self) -> Self;
                fn wrapping_sub(other: Self) -> Self;
                fn wrapping_div(other: Self) -> Self;
                fn wrapping_mul(other: Self) -> Self;
                fn wrapping_pow(other: Self::Exponent) -> Self;
                fn wrapping_rem(other: Self) -> Self;
                fn wrapping_shl(other: u32) -> Self;
                fn wrapping_shr(other: u32) -> Self;
                fn wrapping_div_euclid(other: Self) -> Self;
                fn wrapping_rem_euclid(other: Self) -> Self;

                fn checked_add(other: Self) -> Option<Self>;
                fn checked_sub(other: Self) -> Option<Self>;
                fn checked_div(other: Self) -> Option<Self>;
                fn checked_mul(other: Self) -> Option<Self>;
                fn checked_pow(other: Self::Exponent) -> Option<Self>;
                fn checked_rem(other: Self) -> Option<Self>;
                fn checked_shl(other: Self::Exponent) -> Option<Self>;
                fn checked_shr(other: Self::Exponent) -> Option<Self>;
                /*
                fn unchecked_add(other: Self) -> Option<Self>;
                fn unchecked_mul(other: Self) -> Option<Self>;
                fn unchecked_sub(other: Self) -> Option<Self>;
                */
                fn checked_div_euclid(other: Self) -> Option<Self>;
                fn checked_rem_euclid(other: Self) -> Option<Self>;

                fn overflowing_add(other: Self) -> (Self, bool);
                fn overflowing_sub(other: Self) -> (Self, bool);
                fn overflowing_div(other: Self) -> (Self, bool);
                fn overflowing_mul(other: Self) -> (Self, bool);
                fn overflowing_pow(other: Self::Exponent) -> (Self, bool);
                fn overflowing_rem(other: Self) -> (Self, bool);
                fn overflowing_shl(other: Self::Exponent) -> (Self, bool);
                fn overflowing_shr(other: Self::Exponent) -> (Self, bool);
                fn overflowing_div_euclid(other: Self) -> (Self, bool);
                fn overflowing_rem_euclid(other: Self) -> (Self, bool);
            }

            #[inline]
            fn mul_to_parts(self, other: Self) -> (Self, Self) {
                let res = (self as $double) * (other as $double);
                let ones = ((!(0 as $type)) as $double);
                
                let lower_bits = (res & ones) as $type;
                let higher_bits = ((res >> Self::NUM_BITS) & ones) as $type;
                (higher_bits, lower_bits)
            }
        }
    };
}

derive_unsigned_num_digit!(u8, i8, u16);
derive_unsigned_num_digit!(u16, i16, u32);
derive_unsigned_num_digit!(u32, i32, u64);
derive_unsigned_num_digit!(u64, i64, u128);
// derive_unsigned_num_digit!(u128, i128, u128);
// derive_unsigned_num_digit!(usize, isize, );

derive_signed_num_digit!(i8, u8, i16);
derive_signed_num_digit!(i16, u16, i32);
derive_signed_num_digit!(i32, u32, i64);
derive_signed_num_digit!(i64, u64, i128);
// derive_signed_num_digit!(i128, u128, i128);
// derive_signed_num_digit!(isize, usize);
