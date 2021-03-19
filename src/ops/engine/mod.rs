use crate::num::{BigNum, BigNumMut, DynamicBigNum, UnsignedNumDigit};

mod simple;
pub use self::simple::*;

/// Performs operations on unsigned numbers.
pub trait UnsignedEngine {
    /// Adds `rhs` to `res` and stores result into `res`.
    ///
    /// # Return
    /// Returns true if result size does not fit res(AKA carry flag is set OR `rhs` has some digit set on position higher then res can have).
    fn add_accumulate<D>(res: &mut impl BigNumMut<Digit = D>, rhs: &impl BigNum<Digit = D>) -> bool
    where
        D: UnsignedNumDigit;

    /// Subs `rhs` from `res` and stores result into `res`.
    ///
    /// # Return
    /// Returns 1st true when result size does not fit res.
    /// Returns 2nd true when borrowing from not existing bit occurred(AKA burrow flag is set).
    fn sub_accumulate<D>(res: &mut impl BigNumMut<Digit = D>, rhs: &impl BigNum<Digit = D>) -> bool
    where
        D: UnsignedNumDigit;

    /// Shifts number to the left by `n` bits.
    /// Does not resize number if result does not fit.
    ///
    /// # Retrun
    /// Returns true if result size does not fit `res`.
    fn shift_left_u32<D>(res: &mut impl BigNumMut<Digit = D>, n: u32) -> bool
    where
        D: UnsignedNumDigit;

    /// Shifts number to the left by `n` bits.
    /// Does not resize number if result does not fit.
    ///
    /// # Retrun
    /// Returns true if result size does not fit `res`.
    fn shift_right_u32<D>(res: &mut impl BigNumMut<Digit = D>, n: u32) -> bool
    where
        D: UnsignedNumDigit;

    /// Multiplies `lhs` and `rhs` and adds result of multiplication to res.
    ///
    /// # Note
    /// Multiplication is hard to be done in place without allocations, so there is no function doing it in place.
    ///
    /// # Return
    /// Returns `true` when result does not fit `res`.
    /// False otherwise.
    /// Return value can be thought of as overflow flag.
    fn mul_accumulate<D>(
        res: &mut impl BigNumMut<Digit = D>,
        lhs: &impl BigNum<Digit = D>,
        rhs: &impl BigNum<Digit = D>,
    ) -> bool
    where
        D: UnsignedNumDigit;

    /// Multiplies `lhs` and `rhs` and adds result of multiplication to `res`.
    /// Resizes `res` to fit result.
    ///
    /// # Note
    /// Multiplication is hard to be done in place without allocations, so there is no function doing it in place.
    fn mul_resize<D, M>(res: &mut M, lhs: &impl BigNum<Digit = D>, rhs: &impl BigNum<Digit = D>)
    where
        M: DynamicBigNum<Digit = D> + BigNumMut<Digit = D>,
        D: UnsignedNumDigit;

    /// Subs `rhs` from `res` and stores result into `res`.
    ///
    /// # Return
    /// Returns true when borrowing from not existing bit occurred(AKA burrow flag is set).
    fn sub_resize<D, M>(res: &mut M, rhs: &impl BigNum<Digit = D>) -> bool
    where
        M: DynamicBigNum<Digit = D> + BigNumMut<Digit = D>,
        D: UnsignedNumDigit;

    /// Adds `rhs` to `res` and resizes `res` in order to make it fit result of addition.
    fn add_resize<D, M>(res: &mut M, rhs: &impl BigNum<Digit = D>)
    where
        M: DynamicBigNum<Digit = D> + BigNumMut<Digit = D>,
        D: UnsignedNumDigit;
}
