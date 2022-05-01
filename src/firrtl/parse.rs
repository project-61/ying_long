
use pest::iterators::Pair;
use pest_derive::Parser;

use super::*;


#[derive(Parser)]
#[grammar = "./firrtl/firrtl.pest"]
struct FirrtlParser;

trait Parse {
    fn parse_from(i: Pair<Rule>) -> Self;
}

macro_rules! check {
    ($id:ident, $ast_type:ident) => {
        debug_assert_eq!($id.as_rule(), Rule::$ast_type);
    };
}

macro_rules! start_bind {
    ($id:ident) => {
        let mut $id = $id.into_inner();
    };
}

macro_rules! unbox {
    ($id:ident) => {
        let $id = $id.into_inner().next().unwrap();
    };
}

macro_rules! unbox_collect {
    ($id:ident, $src:expr, $ty:ident) => {
        let $id = $src.into_inner().map($ty::parse_from).collect();
    };
}

macro_rules! next {
    ($id:ident, $src:ident, $ty:ident) => {
        let $id = $ty::parse_from($src.next().unwrap());
    };
}

macro_rules! next_opt {
    ($id:ident, $src:ident, $ty:ident) => {
        let $id = $src.next().map($ty::parse_from);
    };
}

macro_rules! nexts {
    ($id:ident, $src:ident, $ty:ident) => {
        let $id = $src.map($ty::parse_from).collect();
    };
}


impl Parse for Circuit {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, circuit);
        start_bind!(i);
        next!(pos, i, PosInfoOpt);
        next!(id, i, Id);
        nexts!(modules, i, Module);
        Circuit {
            pos,
            id,
            modules,
        }
    }
}

impl Parse for Module {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, circuit);
        start_bind!(i);
        next!(pos, i, PosInfoOpt);
        next!(id, i, Id);
        next!(ports, i, Ports);
        next!(stmts, i, StmtGroup);
        Module {
            pos,
            id,
            ports,
            stmts,
        }
    }
}

impl Parse for PosInfoOpt {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, info_opt);
        start_bind!(i);
        next_opt!(pos, i, PosInfo);
        pos
    }
}

impl Parse for PosInfo {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, info);
        start_bind!(i);
        next!(name, i, Id);
        next!(line, i, Line);
        next!(col, i, Col);
        PosInfo {
            name,
            line,
            col,
        }
    }
}

impl Parse for Id {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, id);
        i.as_str().to_string()
    }
}

impl Parse for Line {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, id);
        let r = i.as_str().parse::<usize>().unwrap();
        Line(r)
    }
}

impl Parse for Col {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, id);
        let r = i.as_str().parse::<usize>().unwrap();
        Col(r)
    }
}

impl Parse for Ports {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, ports);
        unbox_collect!(ports, i, Port);
        ports
    }
}

impl Parse for Port {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, port);
        start_bind!(i);
        next!(pos, i, PosInfoOpt);
        next!(dir, i, Dir);
        next!(bind, i, TypeBind);
        Port {
            pos,
            dir,
            bind,
        }
    }
}

impl Parse for Dir {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, dir);
        if i.as_str() == "input" {
            Dir::Input
        } else {
            // == "output"
            Dir::Output
        }
    }
}

impl Parse for TypeBind {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, type_bind);
        start_bind!(i);
        next!(id, i, Id);
        next!(ty, i, Type);
        TypeBind(id, ty)
    }
}

impl Parse for Type {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, typ);
        start_bind!(i);
        let mut ret = parse_into_raw_type(i.next().unwrap());
        for i in i {
            let sz = VecSize::parse_from(i);
            ret = Type::Vector(Box::new(ret), sz);
        }
        ret
    }
}

#[inline]
fn parse_into_raw_type(i: Pair<Rule>) -> Type {
    check!(i, raw_typ);
    unbox!(i);
    match i.as_rule() {
        Rule::clock => Type::Clock,
        Rule::uint => parse_into_uint(i),
        Rule::sint => parse_into_sint(i),
        Rule::bundle => parse_into_bundle(i),
        _ => unreachable!()
    }
}

#[inline]
fn parse_into_uint(i: Pair<Rule>) -> Type {
    check!(i, uint);
    start_bind!(i);
    next!(size, i, SizeOpt);
    Type::Uint(size)
}

#[inline]
fn parse_into_sint(i: Pair<Rule>) -> Type {
    check!(i, sint);
    start_bind!(i);
    next!(size, i, SizeOpt);
    Type::Sint(size)
}

#[inline]
fn parse_into_bundle(i: Pair<Rule>) -> Type {
    check!(i, bundle);
    unbox_collect!(binds, i, Field);
    Type::Bundle(binds)
}


impl Parse for VecSize {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, type_vec_extend);
        start_bind!(i);
        next!(size, i, usize);
        size
    }
}

impl Parse for SizeOpt {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, size_opt);
        start_bind!(i);
        next_opt!(size, i, usize);
        size
    }
}

impl Parse for Field {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, field);
        start_bind!(i);
        next!(is_flip, i, IsFlip);
        next!(bind, i, TypeBind);
        Field {
            is_flip,
            bind,
        }
    }
}

impl Parse for IsFlip {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, flip_opt);
        !i.as_str().is_empty()
    }
}

impl Parse for StmtGroup {
    fn parse_from(i: Pair<Rule>) -> Self {
        check!(i, stmt_group);
        todo!()
    }
}