use libyinglong::pass::gen_verilog::GenVerilog;
use libyinglong::ylir::*;
use libyinglong::pass::*;
use libyinglong::pass::gen_verilog;
use libyinglong::ylir::type_system::Type;
use libyinglong::ylir::type_system::TypeBind;

fn main() {
    let r = Circuit {
        pos: None,
        id: String::from("Circuit114514"),
        modules: vec![
            Module {
                pos: None,
                id: String::from("Module114514"),
                ports: vec![
                    Port {
                        pos: None,
                        dir: Dir::Input,
                        bind: TypeBind("哼哼哼".to_string(), Type::Uint(Some(32))),
                    },
                    Port {
                        pos: None,
                        dir: Dir::Input,
                        bind: TypeBind("啊啊啊啊啊啊啊啊啊".to_string(), Type::Uint(Some(32))),
                    },
                    Port {
                        pos: None,
                        dir: Dir::Output,
                        bind: TypeBind("你是一个一个端口啊啊啊啊啊啊".to_string(), Type::Uint(Some(32))),
                    }
                ],
                stmts: StmtGroup(vec![
                    Stmt {
                        pos: None,
                        raw_stmt: RawStmt::Inst(
                            String::from("你是一个一个端口啊啊啊啊啊啊"),
                            Expr::Primop(Primop::Add, vec![Expr::Ref("哼哼哼".to_string()), Expr::Ref("啊啊啊啊啊啊啊啊啊".to_string())]),
                        )
                    }
                ])
            },
        ],
    };

    let pm = GenVerilog();
    let out = r.pure_pass(&pm);

    println!("out:\n{}", out);
}
