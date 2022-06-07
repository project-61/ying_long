use libyinglong::pass::gen_verilog::GenVerilog;
use libyinglong::pass::type_infer::GlobalEnv;
use libyinglong::pass::type_infer::TypeCheck;
use libyinglong::ylir::*;
use libyinglong::pass::*;
use libyinglong::pass::gen_verilog;
use libyinglong::ylir::type_system::Type;
use libyinglong::ylir::type_system::TypeBind;

fn main() {
    let mut r = Circuit {
        pos: None,
        id: String::from("Circuit114514"),
        modules: vec![
            Module {
                pos: None,
                id: String::from("Add"),
                ports: vec![
                    Port {
                        pos: None,
                        dir: Dir::Input,
                        bind: TypeBind("a".to_string(), Type::Uint(Some(32))),
                    },
                    Port {
                        pos: None,
                        dir: Dir::Input,
                        bind: TypeBind("b".to_string(), Type::Uint(Some(32))),
                    },
                    Port {
                        pos: None,
                        dir: Dir::Output,
                        bind: TypeBind("c".to_string(), Type::Uint(None)),
                    }
                ],
                stmts: StmtGroup(vec![
                    Stmt {
                        pos: None,
                        raw_stmt: RawStmt::Node(
                            String::from("c"),
                            Expr::Primop(Primop::Add, vec![Expr::Ref("a".to_string()), Expr::Ref("b".to_string())]),
                        )
                    }
                ])
            },
        ],
    };

    let mut pm = GlobalEnv::default();
    r.type_check(&mut pm);
    println!("analysis:\n{:?}", pm);

    // let pm = GenVerilog();
    // let out = r.pure_pass(&pm);

    // println!("{}", out);
}
