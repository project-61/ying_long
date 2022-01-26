use std::fmt::write;

use crate::ylir::Node;

use super::{Module, Pin, Pos, Input, Output};


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
            write!(f, "{}", buf)?;
        }

        write_tab(tabsize+1, f)?;
        write!(f, ");")?;



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
        let pin = &self.0;
        assert_ne!(pin.2, 0);
        write_tab(tabsize, f)?;
        if pin.2 == 1 {
            write!(f, "input {},", pin.0)?;
            pos_gen(&self.1, f)?;
            write!(f, "\n")?;
        } else {
            write!(f, "input [{}:0]\t{},", pin.2-1, pin.0)?;
            pos_gen(&self.1, f)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T: std::fmt::Write> GenVerilog<T> for Output {
    fn gen_verilog(&self, tabsize: u8, f: &mut T) -> std::fmt::Result {
        match &self.0 {
            Node::Pin(pin) => {
                assert_ne!(pin.2, 0);
                write_tab(tabsize, f)?;
                if pin.2 == 1 {
                    write!(f, "output {},", pin.0)?;
                    pos_gen(&self.1, f)?;
                    write!(f, "\n")?;
                } else {
                    write!(f, "output [{}:0]\t{},", pin.2-1, pin.0)?;
                    pos_gen(&self.1, f)?;
                    write!(f, "\n")?;
                }
            },
            Node::Reg(reg) => {
                assert_ne!(pin.2, 0);
                write_tab(tabsize, f)?;
                if pin.2 == 1 {
                    write!(f, "output {},", pin.0)?;
                    pos_gen(&self.1, f)?;
                    write!(f, "\n")?;
                } else {
                    write!(f, "output [{}:0]\t{},", pin.2-1, pin.0)?;
                    pos_gen(&self.1, f)?;
                    write!(f, "\n")?;
                }
            },
            Node::Mem(mem) => todo!(),
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