use std::{collections::HashMap, ops::Neg};

use super::{Dir, GetWidth, Id};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Clock,
    Uint(usize),
    Sint(usize),
    Vector(Vector),
    Bundle(Bundle),
}

impl Type {
    /*
    pub fn unify(&self, other: &Self) -> Option<Type> {
        match (self, other) {
            (r@Type::Uint(Some(sz)), Type::Uint(Some(sz1))) => {
                assert_eq!(sz, sz1, "untyped size mismatch");
                // todo: return None
                Some(r.clone())
            },
            (Type::Uint(_), r@Type::Uint(Some(sz))) |
            (r@Type::Uint(Some(sz)), Type::Uint(_)) => Some(r.clone()),
            (r@Type::Sint(Some(sz)), Type::Sint(Some(sz1))) => {
                assert_eq!(sz, sz1, "untyped size mismatch");
                // todo: return None
                Some(r.clone())
            },
            (Type::Sint(_), r@Type::Sint(Some(sz))) |
            (r@Type::Sint(Some(sz)), Type::Sint(_)) => Some(r.clone()),
            (Type::Vector(a), Type::Vector(b)) => a.unify(b).map(Type::Vector),
            (Type::Bundle(a), Type::Bundle(b)) => a.unify(b).map(Type::Bundle),
            _ => return None,
        }
    }
     */

    pub fn get_vector(&self) -> Option<&Vector> {
        match self {
            Type::Vector(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_bundle(&self) -> Option<&Bundle> {
        match self {
            Type::Bundle(v) => Some(v),
            _ => None,
        }
    }
}

impl GetWidth for Type {
    fn get_width(&self) -> usize {
        match self {
            Type::Clock => 1,
            Type::Uint(s) | Type::Sint(s) => s.clone(),
            Type::Vector(Vector(t, s)) => t.get_width() * s,
            Type::Bundle(Bundle(f)) => f.iter().map(|(_, f)| f.get_width()).sum(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector(pub Box<Type>, pub VecSize);

/*
impl Vector {
    fn unify(&self, other: &Self) -> Option<Self> {
        let t = self.0.as_ref().unify(other.0.as_ref())?;
        if self.1 == other.1 {
            return None;
        }
        Some(Vector(Box::new(t), self.1))
    }
}
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bundle(pub HashMap<Id, Field>);

impl Bundle {
    pub fn get_field(&self, id: &Id) -> Option<&Field> {
        self.0.get(id)
    }
}

impl Neg for Bundle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Bundle(
            self.0
                .iter()
                .map(|(id, f)| (id.clone(), f.clone().neg()))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field {
    pub is_flip: bool,
    pub bind: TypeBind,
}

impl GetWidth for Field {
    fn get_width(&self) -> usize {
        self.bind.get_width()
    }
}

impl Neg for Field {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Field {
            is_flip: !self.is_flip,
            bind: self.bind,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeBind(pub Id, pub Type);

impl GetWidth for TypeBind {
    fn get_width(&self) -> usize {
        self.1.get_width()
    }
}

pub type IsFlip = bool;

pub type VecSize = usize;
// pub type SizeOpt = Option<usize>;
