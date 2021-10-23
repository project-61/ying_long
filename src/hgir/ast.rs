use std::{collections::HashSet, ops::{Add, AddAssign, BitAnd, BitOr, BitXor, Not, Shl}, sync::{Arc, RwLock}};

use crate::utils::Symbol;

use super::{Pin, TriggerType};


#[derive(Debug, Clone)]
pub struct Module {
    pub name: Symbol,
    pub input: Arc<RwLock<HashSet<Pin>>>,
    pub output: Arc<RwLock<HashSet<Pin>>>,
    pub items: Arc<RwLock<Vec<Hardware>>>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Module {
            name: Symbol::from(name),
            input: Default::default(),
            output: Default::default(),
            items: Default::default(),
        }
    }
    pub fn input(&self, pin: &Pin) {
        self.input.write().unwrap().insert(pin.clone());
    }
    pub fn output(&self, pin: &Pin) {
        self.output.write().unwrap().insert(pin.clone());
    }
}

impl AddAssign<Hardware> for Module {
    fn add_assign(&mut self, other: Hardware) {
        self.items.write().unwrap().push(other);
    }
}


#[derive(Debug, Clone)]
pub enum Hardware {
    Assign(Pin, Expr),
    Temporal(TriggerType, Expr),
}

impl Pin {
    pub fn assign(self, expr: Expr) -> Hardware {
        Hardware::Assign(self, expr)
    }
    pub fn S(self) -> Expr {
        Expr::Var(self.0)
    }
}

impl Shl<Expr> for Pin {
    type Output = Hardware;
    fn shl(self, expr: Expr) -> Self::Output {
        self.assign(expr)
    }
}

#[derive(Debug, Clone)]
pub enum TemporalExpr {
    RegSet(Symbol, Expr),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Const(Const),
    Var(Symbol),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Match(Box<Expr>, Vec<MatchCase>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Splitter(Box<Expr>, usize, usize),
    Hub(Vec<Expr>),
}

pub fn C<T>(c: T) -> Expr
where  T: Into<Const> {
    Expr::Const(c.into())
}


impl Add<Expr> for Expr {
    type Output = Expr;
    fn add(self, rhs: Expr) -> Self::Output {
        Expr::Binary(BinaryOp::Add, Box::new(self), Box::new(rhs))
    }
}

impl BitAnd<Expr> for Expr {
    type Output = Expr;
    fn bitand(self, rhs: Expr) -> Self::Output {
        Expr::Binary(BinaryOp::BitAnd, Box::new(self), Box::new(rhs))
    }
}

impl BitOr<Expr> for Expr {
    type Output = Expr;
    fn bitor(self, rhs: Expr) -> Self::Output {
        Expr::Binary(BinaryOp::BitOr, Box::new(self), Box::new(rhs))
    }
}

impl BitXor<Expr> for Expr {
    type Output = Expr;
    fn bitxor(self, rhs: Expr) -> Self::Output {
        Expr::Binary(BinaryOp::BitXor, Box::new(self), Box::new(rhs))
    }
}

impl Not for Expr {
    type Output = Expr;
    fn not(self) -> Self::Output {
        Expr::Unary(UnaryOp::Not, Box::new(self))
    }
}


#[derive(Debug, Clone)]
pub enum Const {
    Int(i64),
    Uint(u64),
    Real(f64),
    Bool(bool),
}

impl From<i32> for Const {
    fn from(i: i32) -> Self {
        Const::Int(i as i64)
    }
}

impl From<i64> for Const {
    fn from(i: i64) -> Self {
        Const::Int(i)
    }
}

impl From<u32> for Const {
    fn from(i: u32) -> Self {
        Const::Uint(i as u64)
    }
}

impl From<u64> for Const {
    fn from(i: u64) -> Self {
        Const::Uint(i)
    }
}

impl From<f32> for Const {
    fn from(i: f32) -> Self {
        Const::Real(i as f64)
    }
}

impl From<f64> for Const {
    fn from(i: f64) -> Self {
        Const::Real(i)
    }
}

impl From<bool> for Const {
    fn from(i: bool) -> Self {
        Const::Bool(i)
    }
}


#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    // Sub,
    // Mul,
    // Div,
    // Mod,
    // LogicAnd,
    // LogicOr,
    BitAnd,
    BitOr,
    BitXor,
    // BitShiftLeft,
    // BitShiftRight,
}

#[derive(Debug, Clone)]
pub enum BitValue {
    Bit(bool),
    X,
    Z,
}

#[derive(Debug, Clone)]
pub enum MatchCase {
    Case(Vec<BitValue>, Vec<Expr>),
    Default(Vec<Expr>),
}
