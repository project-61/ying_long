use std::borrow::Borrow;
use std::fmt::format;

use rayon::prelude::*;

use super::type_infer::GlobalEnv;
use super::PurePass;
use crate::ylir::type_system::*;
use crate::ylir::*;

// pub struct GenVerilog(pub GlobalEnv);

pub trait GenVerilog {
    fn gen_verilog(&self, env: &GlobalEnv) -> String;
}

impl GenVerilog for Circuit {

    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        self.modules
            .par_iter()
            .map(|i| i.gen_verilog(pm))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl GenVerilog for Module {

    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        format!(
            "module {} (\n{}\n\t);\n{}endmodule;",
            self.id,
            self.ports.gen_verilog(pm),
            self.stmts.gen_verilog(pm)
        )
    }
}

impl GenVerilog for Ports {

    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        self.par_iter()
            .map(|p| p.gen_verilog(pm))
            // .collect()
            .collect::<Vec<_>>()
            .join(",\n")
    }
}

impl GenVerilog for Port {
    fn gen_verilog(&self, env: &GlobalEnv) -> String {
        let dir = match self.dir {
            Dir::Input => "input",
            Dir::Output => "output",
            Dir::Inout => "inout",
        };
        format!("\t{} {}", dir, self.bind.gen_verilog(env))
    }
}

impl GenVerilog for StmtGroup {
    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        self.0.par_iter().map(|s| s.gen_verilog(pm)).collect()
        // .collect::<Vec<_>>().join("\n")
    }
}

impl GenVerilog for Stmt {
    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        format!(
            "{}\t{}\n",
            self.raw_stmt.gen_verilog(pm),
            self.pos.gen_verilog(pm)
        )
    }
}

impl GenVerilog for RawStmt {
    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        match self {
            RawStmt::WireDef(w) => w.gen_verilog(pm),
            RawStmt::RegDef(bind, value, append) => todo!(),
            RawStmt::MemDef(memdef) => todo!(),
            RawStmt::Inst(name, value) => todo!(),
            RawStmt::Node(name, value) => format!("\tassign {} = {};", name, value.gen_verilog(pm)),
            RawStmt::Connect(a, b) => todo!(),
            RawStmt::When(w) => w.gen_verilog(pm),
            RawStmt::StmtGroup(sg) => sg.gen_verilog(pm),
        }
    }
}

impl GenVerilog for Expr {
    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        match self {
            Expr::Literal(literal) => todo!(),
            Expr::Ref(id) => id.clone(),
            Expr::SubField(expr, sf) => todo!(),
            Expr::SubIndex(expr, si) => todo!(),
            Expr::SubAccess(expr, sa) => todo!(),
            Expr::Mux(cond, then, else_) => format!(
                "{} ? {} : {}",
                cond.gen_verilog(pm),
                then.gen_verilog(pm),
                else_.gen_verilog(pm)
            ),
            // Expr::Validif(_, _) => todo!(),
            Expr::Primop(op, params) => match op {
                Primop::Add => format!("{} + {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Sub => format!("{} - {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Mul => format!("{} * {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Div => format!("{} / {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Mod => format!("{} % {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Lt => format!("{} < {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Gt => format!("{} > {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Leq => {
                    format!("{} <= {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm))
                }
                Primop::Geq => {
                    format!("{} >= {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm))
                }
                Primop::Eq => format!("{} == {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Neq => {
                    format!("{} != {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm))
                }
                Primop::AsUInt => todo!(),
                Primop::AsSInt => todo!(),
                Primop::AsClock => todo!(),
                // Following four depend on the type system
                Primop::Pad => todo!(),
                Primop::Shl => todo!(), //format!("{} << {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Shr => todo!(), //format!("{} >> {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                // Primop::Dshl    => todo!(),
                // Primop::Dshr    => todo!(),
                Primop::Cvt => todo!(),
                Primop::Neg => format!("-{}", params[0].gen_verilog(pm)),
                Primop::Not => format!("~{}", params[0].gen_verilog(pm)),
                Primop::And => format!("{} & {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Or => format!("{} | {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Xor => format!("{} ^ {}", params[0].gen_verilog(pm), params[1].gen_verilog(pm)),
                Primop::Andr => format!("&{}", params[0].gen_verilog(pm)),
                Primop::Orr => format!("|{}", params[0].gen_verilog(pm)),
                Primop::Xorr => format!("^{}", params[0].gen_verilog(pm)),
                Primop::Cat => format!(
                    "{{{}, {}}}",
                    params[0].gen_verilog(pm),
                    params[1].gen_verilog(pm)
                ),
                Primop::Bits => format!(
                    "{}[{}:{}]",
                    params[0].gen_verilog(pm),
                    params[1].gen_verilog(pm),
                    params[2].gen_verilog(pm)
                ),
                // Primop::Head    => todo!(),
                // Primop::Tail    => format!("{}[{}:0]", params[0].gen_verilog(pm), params[1].gen_verilog(pm), params[2].gen_verilog(pm)),
            },
        }
    }
}

impl GenVerilog for When {
    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        if self.else_.is_none() {
            format!(
                "\tif ({})\n\t\t{}\tend",
                self.cond.gen_verilog(pm),
                self.then.gen_verilog(pm)
            )
        } else {
            format!(
                "\tif ({})\n\t\t{}\n\telse\n\t\t{}\n\tend",
                self.gen_verilog(pm),
                self.then.gen_verilog(pm),
                self.else_.as_ref().unwrap().gen_verilog(pm)
            )
        }
    }
}

impl GenVerilog for TypeBind {
    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        let size = self.1.get_width();
        if size == 0 {
            println!("warning: width is 0");
            return "".to_string();
        } else if size == 1 {
            format!("{}", self.0)
        } else {
            format!("[{}:0]\t{}", size, self.0)
        }
    }
}

impl GenVerilog for PosInfoOpt {
    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        match self {
            PosInfoOpt::None => "".to_string(),
            PosInfoOpt::Some(pos) => pos.gen_verilog(pm),
        }
    }
}

impl GenVerilog for PosInfo {
    fn gen_verilog(&self, pm: &GlobalEnv) -> String {
        // fixme
        format!("@[\"{}\":{:?}:{:?}]", self.file, self.line, self.col)
    }
}
