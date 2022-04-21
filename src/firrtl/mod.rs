pub mod parse;



#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct PosInfo {
    pub name: String,
    pub line: usize,
    pub col: usize,
}


pub type Id = String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeBind (pub Id, pub Type);


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Firrtl {
    pub pos: PosInfo,
    pub id: Id,
    pub modules: Vec<Module>,
    // pub symbol_table: RefCell<HashMap<Id, usize>>,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Module {
    pub pos: PosInfo,
    pub id: Id,
    pub ports: Vec<Port>,
    pub stmt: Stmt,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Port {
    pub pos: PosInfo,
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
    Uint(usize),
    Sint(usize),
    Vector(Box<Type>, usize),
    Bundle(Vec<Field>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Field {
    pub is_flip: bool,
    pub bind: TypeBind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stmt {
    pub pos: PosInfo,
    pub value: RawStmt,
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
    StmtGroup(Vec<Stmt>),
}


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