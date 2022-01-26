pub mod ring_check;
pub mod const_propagate;
pub mod gen_verilog;

use crate::datatype::BitVector;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub pos: Option<Pos>,
    pub clock: bool,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub assigns: Vec<Assign>,
    pub module_instances: Vec<ModuleInstance>,
}

#[derive(Debug, Clone)]
pub struct Input(PinDef, Option<Pos>);

#[derive(Debug, Clone)]
pub struct Output(Node, Option<Pos>);

#[derive(Debug, Clone)]
pub struct ModuleInstance(pub String, pub String, pub Vec<SS>, Option<Pos>);

#[derive(Debug, Clone)]
pub struct Assign(pub PinDef, pub Operator, Option<Pos>);

/// Signal Source
#[derive(Debug, Clone)]
pub enum Constant {
    String(String),
    Int(String, usize),
    Float(String, usize),
    BitVector(BitVector),
}

impl Constant {
    pub fn get_bitvec(&self) -> BitVector {
        match self {
            Constant::BitVector(bv) => bv.clone(),
            _ => todo!(),
        }
    }
}

/// Signal Source
#[derive(Debug, Clone)]
pub enum SS {
    Const(Constant),
    Pin(Pin),
    Reg(Reg),
    Mem(Mem),
}

#[derive(Debug, Clone)]
pub struct Pin(pub String, pub String);

#[derive(Debug, Clone)]
pub struct PinDef(pub String, pub usize);

#[derive(Debug, Clone)]
pub struct Reg(pub String, pub usize);

#[derive(Debug, Clone)]
pub struct Mem(pub Reg, pub usize);

#[derive(Debug, Clone)]
pub enum Node {
    Pin(PinDef),
    Reg(Reg),
    Mem(Mem),
}

#[derive(Debug, Clone)]
pub enum Operator {
    BitAnd(SS, SS),
    BitOr(SS, SS),
    BitXor(SS, SS),
    BitXnor(SS, SS),
    BitNot(SS),
    BitLShift(SS, usize),
    BitRShift(SS, usize),
    // BitURShift(Pin, SS, usize),
    LengthExtend(SS, usize, usize),
    SignalExtend(SS, usize),
    Split(SS, usize, usize),
    BundleReduce(ReduceType, SS),
    Concat(Vec<SS>),
    CmpEq(SS, SS),
    Cond(SS, SS, SS),
    GetField(SS, SS),
}

#[derive(Debug, Clone)]
pub enum ReduceType {
    BitAnd,
    BitOr,
    BitXor,
}


#[derive(Debug, Clone)]
pub struct Pos {
    pub file: String,
    pub line: usize,
    pub col: usize,
}
