use crate::hgir::ast::{C, Expr, Module};

extern crate proc_macro;

mod hgir;
mod utils;

macro_rules! des {
    ($m:ident $($exp:expr)*) => {
        $($m += $exp;)*
    };
}

fn main() {
    let mut modu = Module::new("main");
    let a = pin!();
    let b = pin!();
    // modu += a << C(1i32);
    des!(modu
        b << (C(2i32) | C(4i32))
        a << C(3i32)
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