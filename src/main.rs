mod hgir;
mod utils;

use hgir::ast::{C, Module};
use hgir::Pin;

macro_rules! body {
    ($m:ident $($exp:expr)*) => {
        $($m += $exp;)*
    };
}

macro_rules! pinn {
    ($num:ident) => {
        Pin::from(&format!("pin_{}_{}_{}_{}", module_path!(), stringify!(#exp), line!(), column!()), 1)
    };
    ($exp:ident:$num:expr) => {
        Pin::from(&format!("pin_{}_{}_{}_{}", module_path!(), stringify!(#exp), line!(), column!()), $num)
    };
}

macro_rules! inputs {
    ($m:ident $(, $exp:ident$(:$num:expr)?)*) => {
        $(
            let $exp = pinn!($exp $(:$num)?);
            $m.input(&$exp);
        )*
    };
}

macro_rules! outputs {
    ($m:ident $(, $exp:ident$(:$num:expr)?)*) => {
        $(
            let $exp = pinn!($exp$(:$num)?);
            $m.output(&$exp);
        )*
    };
}

macro_rules! module {
    ($m:ident
        $(input $($input:ident$(:$inum:expr)?),+ $(,)?;)?
        $(output $($output:ident$(:$onum:expr)?),+ $(,)?;)?
        // $(reg $($reg:ident$(:$rnum:expr)?),+ $(,)?;)?
        body $($x:expr)+) => {
        let mut $m = Module::new(stringify!(#m));
        $(inputs!($m $(,$input$(:$inum)?)+);)?
        $(outputs!($m $(,$output$(:$onum)?)+);)?
        $($m += $x;)+
    };
}

fn main_module() -> Module {
    module!{sig114
        input a:11, b, d, e;
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
    module_path!();
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