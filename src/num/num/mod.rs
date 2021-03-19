use crate::num::UnsignedNumDigit;
use core::ops::{Index, IndexMut};

/// Default type of bignum.
pub type DefaultBigNumDigit = u32;

mod vec;
pub use self::vec::*;

/// Single number, which is split into digits.
pub trait BigNum: Sized + Clone + Index<usize, Output = Self::Digit> {
    /// Type of digit of this bignum.
    type Digit: UnsignedNumDigit;

    /// Creates new bignum with specified size(zero for dynamic ones), which contains value zero.
    fn new_zeroed() -> Self;

    /// Returns nth digit.
    /// Digits are ordered form least significant to the most significant one AKA little endian.
    /// Its synonymous to the index
    fn get_digit(&self, pos: usize) -> Self::Digit;

    /// Returns amount of digits of this bignum.
    fn len(&self) -> usize;
}

/// BigNum, which is not immutable and can be mutated.
pub trait BigNumMut: BigNum + IndexMut<usize> {
    /// Sets specified digit at nth position.
    fn set_digit(&mut self, pos: usize, digit: Self::Digit);
}

/// BigNum, which can be resized.
pub trait DynamicBigNum: BigNum {
    /// Resizes bignum to have res digits.
    ///
    /// # 1. `new_sz < sz`
    /// Number is truncated to the `new_res` least significant digits.
    /// Just like truncating cast in C do.
    ///
    /// # 2. `new_sz = sz`
    /// This one is noop.
    ///
    /// # 3. `new_sz > sz`
    /// Number is padded with zeros.
    fn resize(&mut self, res: usize);

    /// Creates new bignum with value zero and specified size.
    fn new_zeroed_sized(size: usize) -> Self;
}

/// BigNum, which has sign.
pub trait SignedBigNum: BigNum {
    /// True if minus flag is set.
    /// Note: it may be true even if value is zero.
    fn is_minus(&self) -> bool;

}

/// SignedBigNum, which is mutable.
pub trait SignedBigNumMut: SignedBigNum + BigNumMut {
    /// Sets minus sign for this bignum.
    fn set_minus(&mut self, minus: bool);
}