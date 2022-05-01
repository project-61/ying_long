// use std::cell::RefCell;

pub mod parse;


#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct PosInfo {
    pub name: String,
    pub line: Line,
    pub col: Col,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Line (pub usize);
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Col(pub usize);

pub type PosInfoOpt = Option<PosInfo>;

pub type Id = String;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeBind (pub Id, pub Type);


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
pub enum Type {
    Clock,
    Uint(SizeOpt),
    Sint(SizeOpt),
    Vector(Box<Type>, VecSize),
    Bundle(Vec<Field>),
}

pub type VecSize = usize;
pub type SizeOpt = Option<usize>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field {
    pub is_flip: IsFlip,
    pub bind: TypeBind,
}

pub type IsFlip = bool;

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
    Inst(Id, Id),
    Node(Id, Expr),
    Connect(Expr, Expr),
    PartialConnect(Expr, Expr),
    Invalidate(Expr),
    When(Box<When>),
    Stop(Stop),
    Printf(Printf),
    Skip,
    StmtGroup(StmtGroup),
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stop(pub Expr, pub Expr, pub usize);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Printf(pub Expr, pub Expr, pub String, pub Vec<Expr>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ref(Id),
    SubField(Box<Expr>, Id),
    SubIndex(Box<Expr>, usize),
    SubAccess(Box<Expr>, Box<Expr>),
    Mux(Box<Expr>, Box<Expr>, Box<Expr>),
    Validif(Box<Expr>, Box<Expr>),
    Primop(Primop, Vec<Expr>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Literal {
    pub tp: Type,
    pub value: LiteralValue,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LiteralValue {
    Int(usize),
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
    Dshl,
    Dshr,
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
    Head,
    Tail,
}