use super::{GetWidth, Id};



#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Clock,
    Uint(SizeOpt),
    Sint(SizeOpt),
    Vector(Box<Type>, VecSize),
    Bundle(Vec<Field>),
}

impl GetWidth for Type {
    fn get_width(&self) -> Option<usize> {
        match self {
            Type::Clock => Some(1),
            Type::Uint(s) => s.clone(),
            Type::Sint(s) => s.clone(),
            Type::Vector(t, s) => t.get_width().map(|w| w*s),
            Type::Bundle(f) => f.iter().map(|f| f.get_width()).sum(),
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field {
    pub is_flip: IsFlip,
    pub bind: TypeBind,
}

impl GetWidth for Field {
    fn get_width(&self) -> Option<usize> {
        self.bind.get_width()
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeBind (pub Id, pub Type);

impl GetWidth for TypeBind {
    fn get_width(&self) -> Option<usize> {
        self.1.get_width()
    }
}


pub type IsFlip = bool;

pub type VecSize = usize;
pub type SizeOpt = Option<usize>;