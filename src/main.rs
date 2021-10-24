use crate::hgir::ast::{C, Module};

extern crate proc_macro;

mod hgir;
mod utils;

macro_rules! body {
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

macro_rules! module {
    ($m:ident
        $(input $($input:ident),+ $(,)?;)?
        $(output $($output:ident),+ $(,)?;)?
        // $(reg $($reg:ident:$rego:expr),+ $(,)?;)?
        body $($x:expr)+) => {
        let mut $m = Module::new(stringify!(#m));
        $(inputs!($m $(,$input)+);)?
        $(outputs!($m $(,$output)+);)?
        $($m += $x;)+
    };
}

fn main_module() -> Module {
    module!{sig114
        input a, b, d, e;
        output c, f;
        body
            c << (a.S() | b.S())
            f << (d.S() + e.S())
    };
    sig114
}

fn main() {
    let m = main_module();
    // m.compile().emit(Verilog::new());
    println!("make: {:?}", m);
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