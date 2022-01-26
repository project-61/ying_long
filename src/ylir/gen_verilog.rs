use crate::ylir::Node;

use super::{Module, Pos, Input, Output, PinDef, Reg, Mem};


pub trait GenVerilog<T: std::fmt::Write> {
    fn gen_verilog(&self, tabsize: u8, f: &mut T) -> std::fmt::Result;
}

impl<T: std::fmt::Write> GenVerilog<T> for Module {
    fn gen_verilog(&self, tabsize: u8, f: &mut T) -> std::fmt::Result {
        write_tab(tabsize, f)?;
        write!(f, "module {}(\n", self.name)?;

        let mut buf = String::new();
        for i in self.inputs.iter() {
            i.gen_verilog(tabsize+1, &mut buf)?;
        }
        if buf != "" {
            buf.pop();
            buf.pop();
            buf.push('\n');
            write!(f, "{}", buf)?;
        }

        write_tab(tabsize+1, f)?;
        write!(f, ");\n")?;



        write_tab(tabsize, f)?;
        write!(f, "endmodule")?;
        Ok(())
    }
}

fn pos_gen<T: std::fmt::Write>(pos: &Option<Pos>, f: &mut T) -> std::fmt::Result {
    if let Some(p) = pos {
        write_tab(1, f)?;
        write!(f, "// '{}', {}:{}\n", p.file, p.line, p.col)?;
    }
    Ok(())
}

impl<T: std::fmt::Write> GenVerilog<T> for Input {
    fn gen_verilog(&self, tabsize: u8, f: &mut T) -> std::fmt::Result {
        write_tab(tabsize, f)?;
        write!(f, "input")?;
        self.0.gen_verilog(tabsize, f)?;
        pos_gen(&self.1, f)?;
        write!(f, "\n")?;
        Ok(())
    }
}

impl<T: std::fmt::Write> GenVerilog<T> for Output {
    fn gen_verilog(&self, tabsize: u8, f: &mut T) -> std::fmt::Result {
        write_tab(tabsize, f)?;
        write!(f, "output")?;
        match &self.0 {
            Node::Pin(pin) => {
                pin.gen_verilog(tabsize+1, f)?;
            },
            Node::Reg(reg) => {
                reg.gen_verilog(tabsize+1, f)?;
            },
            Node::Mem(mem) => {
                mem.gen_verilog(tabsize+1, f)?;
            },
        }
        pos_gen(&self.1, f)?;
        write!(f, "\n")?;
        Ok(())
    }
}

impl<T: std::fmt::Write> GenVerilog<T> for PinDef {
    fn gen_verilog(&self, _: u8, f: &mut T) -> std::fmt::Result {
        let pin = self;
        assert_ne!(pin.2, 0);
        let arr_str = if pin.2 > 1 {
            format!("[{}:{}]", 0, pin.2-1)
        } else {
            "".to_string()
        };
        if pin.1 == 1 {
            write!(f, " {}{},", pin.0, arr_str)?;
        } else {
            write!(f, " [{}:0]\t{}{},", pin.1-1, pin.0, arr_str)?;
        }
        Ok(())
    }
}

impl<T: std::fmt::Write> GenVerilog<T> for Reg {
    fn gen_verilog(&self, _: u8, f: &mut T) -> std::fmt::Result {
        let reg = self;
        assert_ne!(reg.1, 0);
        if reg.1 == 1 {
            write!(f, " reg {},", reg.0)?;
        } else {
            write!(f, " reg [{}:0]\t{},", reg.1-1, reg.0)?;
        }
        Ok(())
    }
}

impl<T: std::fmt::Write> GenVerilog<T> for Mem {
    fn gen_verilog(&self, _: u8, f: &mut T) -> std::fmt::Result {
        let reg = &self.0;
        let arr_str = if self.1 > 1 {
            format!("[{}:{}]", 0, self.1-1)
        } else {
            "".to_string()
        };
        assert_ne!(reg.1, 0);
        if reg.1 == 1 {
            write!(f, " reg {}{},", reg.0, arr_str)?;
        } else {
            write!(f, " reg [{}:0]\t{}{},", reg.1-1, reg.0, arr_str)?;
        }
        Ok(())
    }
}

fn write_tab<T: std::fmt::Write>(tabsize: u8, f: &mut T) -> std::fmt::Result {
    for _ in 0..tabsize {
        write!(f, "{}", "\t")?;
    }
    Ok(())
}