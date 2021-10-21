use std::{array, ops::{BitAnd, BitOr, BitXor, Shl, Shr}};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bits(pub Vec<bool>);

impl BitAnd for Bits {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl BitOr for Bits {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl BitXor for Bits {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl Shl<usize> for Bits {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        todo!()
    }
}

impl Shr<usize> for Bits {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        todo!()
    }
}

impl Bits {
    pub fn get_uint<R>(&self) -> u64 {
        todo!()
    }
}
