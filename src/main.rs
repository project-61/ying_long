use libyinglong::ylir::{Module, PinDef, Input};
use libyinglong::ylir::gen_verilog::GenVerilog;

fn main() {
    let modu = Module {
        name: "mod114".to_string(),
        pos: None,
        clock: false,
        inputs: vec![Input(PinDef("mem".to_string(), 8, 114514), None), Input(PinDef("mcode".to_string(), 32, 1), None)],
        outputs: vec![],
        assigns: vec![],
        module_instances: vec![],
    };
    let mut output = String::new();
    modu.gen_verilog(0, &mut output).unwrap();
    println!("{}", output);
}
