use super::*;

pub trait IsNone {
    fn is_none(&self) -> bool;
}

impl IsNone for Constant {
    fn is_none(&self) -> bool {
        match self {
            Constant::None => true,
            _ => false,
        }
    }
}

impl IsNone for RawPin {
    fn is_none(&self) -> bool {
        if let RawPin::Const(c) = self {
            c.is_none()
        } else {
            false
        }
    }
}

impl IsNone for Pin {
    fn is_none(&self) -> bool {
        self.0.is_none()
    }
}