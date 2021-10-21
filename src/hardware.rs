// ast
pub mod is_none;

use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::datatypes::Bits;



#[derive(Debug, Clone)]
pub struct Input(pub Arc<RwLock<Pin>>);


#[derive(Debug, Clone)]
pub struct IO {
    pub input: HashMap<String, Input>,
    pub output: HashMap<String, Pin>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Option<String>,
    pub io: IO,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pin(Arc<RawPin>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawPin {
    And(Pin, Pin),
    Or(Pin, Pin),
    Xor(Pin, Pin),
    Const(Constant),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    None, // 高阻
    Uint(Bits),
}
