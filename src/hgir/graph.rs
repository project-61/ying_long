use std::{collections::{HashMap, HashSet}, sync::Arc};

use crate::utils::Symbol;

use super::{Pin, TriggerType};



/// Hardware Graph IR


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wire(pub Symbol, pub Symbol); // input, output

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gate(pub Symbol, pub Symbol, pub Symbol); // input1, input2, output

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Splitter(pub Symbol, pub Symbol, pub Symbol); // input, letf[, right], output

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hub(pub Vec<Symbol>, pub Symbol); // inputs, output

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reg(pub Symbol, pub Symbol, pub Symbol); // clock, input, output

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Clock(pub Symbol);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trigger(TriggerType, ); // todo


#[derive(Debug, Clone)]
pub struct Module {
    pub name: Symbol,
    pub input: HashMap<Symbol, Pin>,
    pub output: HashMap<Symbol, Pin>,
    pub pin_map: HashMap<Symbol, Pin>,
    pub wire: Vec<Wire>,
    pub gate: HashSet<Gate>,
    pub splitter: HashSet<Splitter>,
    pub hub: HashSet<Hub>,
    // Temporal logic
    pub reg: HashMap<Arc<String>, Reg>,
    pub trigger: Vec<Trigger>,
    // pub clocks: HashMap<Arc<String>, Clock>,
}
