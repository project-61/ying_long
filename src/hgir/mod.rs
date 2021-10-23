pub mod ast;
pub mod graph;
pub mod ast_into_graph;
pub mod frontend;

use crate::utils::Symbol;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pin(Symbol, usize);

impl Pin {
    pub fn new(size: usize) -> Pin {
        Pin(Symbol::new(), size)
    }
    pub fn from(i: &str, size: usize) -> Self {
        Pin(Symbol::from(i), size)
    }

    pub fn size(&self) -> usize {
        self.1
    }
}

#[macro_export]
macro_rules! pin {
    () => {
        crate::hgir::Pin::new(1)
    };
    ($size:expr) => {
        crate::hgir::Pin::new($size)
    };
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