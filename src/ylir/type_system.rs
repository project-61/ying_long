use super::{GetWidth, Id};



#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Clock,
    Uint(SizeOpt),
    Sint(SizeOpt),
    Vector(Vector),
    Bundle(Bundle),
}

impl Type {
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
    fn get_width(&self) -> Option<usize> {
        match self {
            Type::Clock => Some(1),
            Type::Uint(s) => s.clone(),
            Type::Sint(s) => s.clone(),
            Type::Vector(Vector(t, s)) => t.get_width().map(|w| w*s),
            Type::Bundle(Bundle(f)) => f.iter().map(|f| f.get_width()).sum(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vector (pub Box<Type>, pub VecSize);

#[derive(Debug, Clone)]
pub struct Bundle (pub Vec<Field>);

impl Bundle {
    pub fn get_field(&self, id: &Id) -> Option<&Field> {
        self.0.iter().find(|f| f.bind.0 == *id)
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