use crate::num::{BigNum, BigNumMut, UnsignedNumDigit};
use crate::ops::UnsignedEngine;

pub trait BigNumExt {
    /// Adds other to self into newly allocated result.
    ///
    /// # Panic
    /// Panics when
    fn add<E, D>(&self, other: &impl BigNum<Digit = D>) -> Self
    where
        Self: BigNumMut<Digit = D>,
        D: UnsignedNumDigit,
        E: UnsignedEngine;
}

impl<T> BigNumExt for T
where
    T: BigNum,
{
    fn add<E, D>(&self, other: &impl BigNum<Digit = D>) -> Self
    where
        Self: BigNumMut<Digit = D>,
        D: UnsignedNumDigit,
        E: UnsignedEngine,
    {
        let mut res = self.clone();
        let _carry = E::add_accumulate(&mut res, other);
        res
    }
}
