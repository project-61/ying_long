use crate::hgir::ast::{C, Expr, Module};

extern crate proc_macro;

mod hgir;
mod utils;

macro_rules! des {
    ($m:ident $($exp:expr)*) => {
        $($m += $exp;)*
    };
}

macro_rules! inputs {
    ($m:ident $(, $exp:ident)*) => {
        $(
            let $exp = pin!();
            $m.input(&$exp);
        )*
    };
}

macro_rules! outputs {
    ($m:ident $(, $exp:ident)*) => {
        $(
            let $exp = pin!();
            $m.output(&$exp);
        )*
    };
}

fn main() {
    let mut modu = Module::new("main");
    inputs!(modu, a, b);
    outputs!(modu, c);
    des!(modu
        c << (a.S() | b.S() | C(2i32))
    );
    println!("Hello, world!");
}


#[test]
fn test_pin() {
    let pin = pin!();
    assert_eq!(pin.size(), 1);
    let pin = pin!(8);
    assert_eq!(pin.size(), 8);
    let pin = pin!(114514);
    assert_eq!(pin.size(), 114514);
}