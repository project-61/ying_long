use std::collections::HashSet;

use crate::utils::Symbol;

use super::{Pin, TriggerType};


#[derive(Debug, Clone)]
pub struct Module {
    pub name: Symbol,
    pub input: HashSet<Pin>,
    pub output: HashSet<Pin>,
    pub items: Vec<Hardware>,
}

#[derive(Debug, Clone)]
pub enum Hardware {
    Assign(Symbol, Expression),
    Temporal(TriggerType, Expression),
}

#[derive(Debug, Clone)]
pub enum TemporalExpr {
    RegSet(Symbol, Expression),
    Expr(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(Constant),
    Variable(Symbol),
    Unary(UnaryOp, Box<Expression>),
    Binary(BinaryOp, Box<Expression>, Box<Expression>),
    Match(Box<Expression>, Vec<MatchCase>),
}

#[derive(Debug, Clone)]
pub enum Constant {
    Int(i64),
    Uint(u64),
    Real(f64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    LogicNot,
    BitNot,
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
    Case(Vec<BitValue>, Vec<Expression>),
    Default(Vec<Expression>),
}
