use std::ops::{BitAnd, BitOr, BitXor};



#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum BitValue {
    Zero = 0,
    One = 1,
    X = 2,
    // Z = 3,
}

impl BitAnd for BitValue {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BitValue::One, BitValue::One) => BitValue::One,
            (BitValue::Zero, BitValue::Zero)|
            (BitValue::Zero, BitValue::One) |
            (BitValue::One, BitValue::Zero) |
            (BitValue::Zero, BitValue::X)   |
            (BitValue::X, BitValue::Zero) => BitValue::Zero,
            (BitValue::One, BitValue::X)    |
            (BitValue::X, BitValue::One)    |
            (BitValue::X, BitValue::X) => BitValue::X,
        }
    }
}

impl BitOr for BitValue {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BitValue::Zero, BitValue::Zero) => BitValue::Zero,
            (BitValue::Zero, BitValue::One) |
            (BitValue::One, BitValue::Zero) |
            (BitValue::One, BitValue::One)  |
            (BitValue::One, BitValue::X)    |
            (BitValue::X, BitValue::One) => BitValue::One,
            (BitValue::Zero, BitValue::X) |
            (BitValue::X, BitValue::Zero) |
            (BitValue::X, BitValue::X) => BitValue::X,
        }
    }
}

impl BitXor for BitValue {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BitValue::Zero, BitValue::Zero) => BitValue::Zero,
            (BitValue::One, BitValue::One) => BitValue::Zero,
            (BitValue::Zero, BitValue::One) |
            (BitValue::One, BitValue::Zero) => BitValue::One,
            _ => BitValue::X,
        }
    }
}

pub const XSTATE: BitValue = BitValue::X;
// pub const ZSTATE: BitValue = BitValue::Z;

pub type BitVector = Vec<BitValue>;

pub type MemValue = Vec<BitValue>;
// pub type MemValue = Vec<BitVector>;
