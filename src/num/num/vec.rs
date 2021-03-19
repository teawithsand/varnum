use crate::num::{BigNum, BigNumMut, DefaultBigNumDigit, DynamicBigNum, UnsignedNumDigit};
use core::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct VecBigNum<D = DefaultBigNumDigit> {
    num: Vec<D>,
}

impl<D> VecBigNum<D> {
    pub fn into_inner(self) -> Vec<D> {
        self.num
    }
}
impl<D> From<Vec<D>> for VecBigNum<D> {
    /// Creates number from little endian digits.
    fn from(num: Vec<D>) -> Self {
        Self { num }
    }
}

impl<D> Into<Vec<D>> for VecBigNum<D> {
    fn into(self) -> Vec<D> {
        self.num
    }
}

impl<D> Index<usize> for VecBigNum<D> {
    type Output = D;

    #[inline]
    fn index(&self, index: usize) -> &D {
        &self.num[index]
    }
}

impl<D> IndexMut<usize> for VecBigNum<D> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut D {
        &mut self.num[index]
    }
}

impl<D> BigNumMut for VecBigNum<D>
where
    D: UnsignedNumDigit,
{
    #[inline]
    fn set_digit(&mut self, pos: usize, digit: Self::Digit) {
        self.num[pos] = digit;
    }
}

impl<D> BigNum for VecBigNum<D>
where
    D: UnsignedNumDigit,
{
    type Digit = D;

    #[inline]
    fn new_zeroed() -> Self {
        Self { num: Vec::new() }
    }

    #[inline]
    fn get_digit(&self, pos: usize) -> Self::Digit {
        *self.index(pos)
    }

    #[inline]
    fn len(&self) -> usize {
        self.num.len()
    }
}

impl<D> DynamicBigNum for VecBigNum<D>
where
    D: UnsignedNumDigit,
{
    #[inline]
    fn resize(&mut self, res: usize) {
        self.num.resize(res, D::default())
    }

    #[inline]
    fn new_zeroed_sized(size: usize) -> Self {
        let mut num = Vec::new();
        num.resize(size, D::default());
        Self { num }
    }
}
