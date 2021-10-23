pub mod ast;
pub mod graph;
pub mod ast_into_graph;

use crate::utils::Symbol;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pin(pub Symbol, usize);

impl Pin {
    pub fn new(size: usize) -> Pin {
        Pin(Symbol::new(), size)
    }
    fn from(i: &str, size: usize) -> Self {
        Pin(Symbol::from(i), size)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TriggerType {
    Init,
    Always(Vec<EventType>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    Any(Symbol),
    Posedge(Symbol),
    Negedge(Symbol),
}