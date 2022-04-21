
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./firrtl/firrtl.pest"]
struct FirrtlParser;
