// use std::cell::RefCell;

use std::{ops::Neg, collections::HashMap};

use self::type_system::{Type, TypeBind};

pub mod type_system;
// pub mod parse;

pub trait GetWidth {
    fn get_width(&self) -> usize;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Circuit {
    pub pos: PosInfoOpt,
    pub id: Id,
    pub modules: Vec<Module>,
    // pub symbol_table: RefCell<HashMap<Id, usize>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Module {
    pub pos: PosInfoOpt,
    pub id: Id,
    // pub is_gen: bool,
    pub ports: Ports,

    pub wire_defs: HashMap<Id, WireDef>,
    pub reg_defs: HashMap<Id, RegDef>,
    pub mem_defs: HashMap<Id, Mem>,

    // inst id, module id,
    pub module_insts: HashMap<Id, Id>,
    pub connects: Vec<(Expr, Expr)>,

    pub nodes: HashMap<Id, Expr>,
    // pub whens: Vec<When>,
}

impl Module {
    pub fn is_wire(&self, id: &Id) -> bool {
        self.wire_defs.contains_key(id)
    }
}

/*
impl Module {

    pub fn is_uninstenced(&self) -> bool {
        self.ports.iter().any(|x| matches!(x.bind.1, Type::Uint(_) | Type::Sint(_)))
    }
}
*/

pub type Ports = Vec<Port>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Port {
    pub pos: PosInfoOpt,
    pub dir: Dir,
    pub bind: TypeBind,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Dir {
    Input = 01,
    Output = 10,
    Inout = 11,
}

impl Neg for Dir {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Dir::Input => Dir::Output,
            Dir::Output => Dir::Input,
            Dir::Inout => Dir::Inout,
        }
    }
}

impl Dir {
    #[inline(always)]
    pub fn is_input(self) -> bool {
        let r = self as u8 & Dir::Input as u8;
        r != 0
    }

    #[inline(always)]
    pub fn is_output(self) -> bool {
        let r = self as u8 & Dir::Output as u8;
        r != 0
    }

    #[inline(always)]
    pub fn is_inout(self) -> bool {
        let r = self as u8 & Dir::Inout as u8;
        r != 0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stmt {
    pub pos: PosInfoOpt,
    pub raw_stmt: RawStmt,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RawStmt {
    WireDef(WireDef),
    RegDef(RegDef),
    MemDef(Mem),
    Inst(Id, Id),
    Node(Id, Expr),
    Connect(Expr, Expr),
    // PartialConnect(Expr, Expr),
    // When(Box<When>),
    StmtGroup(StmtGroup),
    // Printf(Printf),
    // Invalidate(Expr),
    // Stop(Stop),
    // Skip,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WireDef (pub TypeBind);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RegDef {
    pub bind: TypeBind,
    pub clk: Expr,
    // rst, value
    pub reset: Option<(Expr, Expr)>
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ref(Id),
    SubField(Box<Expr>, Id),
    SubIndex(Box<Expr>, usize),
    SubAccess(Box<Expr>, Box<Expr>),
    Mux(Box<Expr>, Box<Expr>, Box<Expr>),
    // Validif(Box<Expr>, Box<Expr>),
    Primop(Primop, Vec<Expr>),
}

impl Expr {
    pub fn get_literal(&self) -> Option<&Literal> {
        match self {
            Expr::Literal(literal) => Some(literal),
            _ => None,
        }
    }

    pub fn get_id(&self) -> Option<&Id> {
        match self {
            Expr::Ref(id) => Some(id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StmtGroup(pub Vec<Stmt>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Mem {
    pub id: Id,
    pub data_type: Type,
    pub depth: usize,
    pub read_latency: usize,
    pub write_latency: usize,
    pub read_under_write: Ruw,
    pub reader: Vec<Id>,
    pub writer: Vec<Id>,
    pub readwriter: Vec<Id>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Ruw {
    Undefined,
    New,
    Old,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct When {
    pub cond: Expr,
    pub then: Stmt,
    pub else_: Option<Stmt>,
}

/*
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stop(pub Expr, pub Expr, pub usize);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Printf(pub Expr, pub Expr, pub String, pub Vec<Expr>);
//  */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Literal {
    pub typ: Type,
    pub value: LiteralValue,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LiteralValue {
    Int(u128),
    String(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Primop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lt,
    Leq,
    Gt,
    Geq,
    Eq,
    Neq,
    Pad,
    AsUInt,
    AsSInt,
    AsClock,
    Shl,
    Shr,
    // Dshl,
    // Dshr,
    Cvt,
    Neg,
    Not,
    And,
    Or,
    Xor,
    Andr,
    Orr,
    Xorr,
    Cat,
    Bits,
    // Head,
    // Tail,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct PosInfo {
    pub file: String,
    pub line: Line,
    pub col: Col,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Line(pub usize);
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Col(pub usize);

pub type PosInfoOpt = Option<PosInfo>;

pub type Id = String;
