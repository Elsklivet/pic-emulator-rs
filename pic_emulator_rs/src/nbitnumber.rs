use std::ops;

pub struct NBitNumber<const N: usize> {
    pub value: u16,
}

pub trait NumberOperations<const N: usize> {
    fn get_max() -> Self;
    fn as_u16(&self) -> u16;
    fn as_usize(&self) -> usize;
    fn get(&self) -> u16;
    fn new() -> Self;

}

pub trait BitwiseOperations {
    fn bitwise_and_with_16(&self, rhs: u16) -> Self;
    fn bitwise_and_with_32(&self, rhs: u32) -> Self;
}

impl<const N: usize> NBitNumber<N> {
    pub fn new(value: u16) -> Self {
        NBitNumber { value: value & ((1 << N) - 1)}
    }

    pub fn get(&self) -> u16 {
        self.value
    }
}

impl<const N: usize> NumberOperations<N> for NBitNumber<N> {
    fn new() -> Self {
        NBitNumber::<N>::new(0)
    }

    fn get(&self) -> u16 {
        self.value
    }

    fn get_max() -> Self {
        NBitNumber::<N>::new((1 << N) - 1)
    }

    fn as_u16(&self) -> u16 {
        self.value
    }

    fn as_usize(&self) -> usize {
        self.value as usize
    }
}

impl<const N: usize> BitwiseOperations for NBitNumber<N> {
    fn bitwise_and_with_16(&self, rhs: u16) -> Self {
        NBitNumber::<N>::new(self.value & rhs)
    }


    fn bitwise_and_with_32(&self, rhs: u32) -> Self {
        NBitNumber::<N>::new(self.value & rhs as u16)
    }
}

impl<const N: usize> Clone for NBitNumber<N> {
    fn clone(&self) -> Self {
        NBitNumber::<N>::new(self.value)
    }
}

impl<const N: usize> ops::Add<usize> for NBitNumber<N> {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
        NBitNumber::<N>::new(self.value + rhs as u16)
    }
}


impl<const N: usize> Copy for NBitNumber<N> {}


pub type u12 = NBitNumber<12>;
pub type u7 = NBitNumber<7>;
pub type u9 = NBitNumber<9>;