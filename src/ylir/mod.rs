// use std::cell::RefCell;

use self::type_system::{TypeBind, Type};

pub mod type_system;
// pub mod parse;


pub trait GetWidth {
    fn get_width(&self) -> Option<usize>;
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
    pub ports: Ports,
    pub stmts: StmtGroup,
}

pub type Ports = Vec<Port>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Port {
    pub pos: PosInfoOpt,
    pub dir: Dir,
    pub bind: TypeBind,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Dir {
    Input,
    Output,
}




#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stmt {
    pub pos: PosInfoOpt,
    pub raw_stmt: RawStmt,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RawStmt {
    WireDef(TypeBind),
    RegDef(TypeBind, Expr, Option<(Expr, Expr)>),
    MemDef(Mem),
    Inst(Id, Expr),
    Node(Id, Expr),
    Connect(Expr, Expr),
    // PartialConnect(Expr, Expr),
    When(Box<When>),
    StmtGroup(StmtGroup),
    // Printf(Printf),
    // Invalidate(Expr),
    // Stop(Stop),
    // Skip,
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
    pub tp: Type,
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
pub struct Line (pub usize);
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Col(pub usize);

pub type PosInfoOpt = Option<PosInfo>;



pub type Id = String;
