pub mod ast;
pub mod graph;
pub mod ast_into_graph;

use crate::utils::Symbol;


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