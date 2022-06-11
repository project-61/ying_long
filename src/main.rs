use libyinglong::pass::gen_verilog;
use libyinglong::pass::gen_verilog::GenVerilog;
use libyinglong::pass::type_infer::GlobalEnv;
use libyinglong::pass::type_infer::TypeCheck;
use libyinglong::pass::*;
use libyinglong::ylir::type_system::Type;
use libyinglong::ylir::type_system::TypeBind;
use libyinglong::ylir::*;

fn main() {
    /*
    let mut r = Circuit {
        pos: None,
        id: "Circuit114514".to_string(),
        modules: vec![
            Module {
                pos: None,
                is_gen: false,
                id: "Add".to_string(),
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
                            "c".to_string(),
                            Expr::Primop(Primop::Add, vec![Expr::Ref("a".to_string()), Expr::Ref("b".to_string())]),
                        )
                    }
                ])
            },
            Module {
                pos: None,
                id: "top".to_string(),
                is_gen: false,
                ports: vec![
                    Port {
                        pos: None,
                        dir: Dir::Output,
                        bind: TypeBind("out".to_string(), Type::Uint(None)),
                    }
                ],
                stmts: StmtGroup(vec![
                    Stmt {
                        pos: None,
                        raw_stmt: RawStmt::Inst(
                            "Add".to_string(),
                            "add".to_string(),
                        )
                    },
                    Stmt {
                        pos: None,
                        raw_stmt: RawStmt::Connect(
                            Expr::SubField(Box::new(Expr::Ref("add".to_string())), "a".to_string()),
                            Expr::Literal(Literal { tp: Type::Uint(Some(32)), value: LiteralValue::Int(1) }),
                        )
                    },
                    Stmt {
                        pos: None,
                        raw_stmt: RawStmt::Connect(
                            Expr::SubField(Box::new(Expr::Ref("add".to_string())), "b".to_string()),
                            Expr::Literal(Literal { tp: Type::Uint(Some(32)), value: LiteralValue::Int(2) }),
                        )
                    },
                    Stmt {
                        pos: None,
                        raw_stmt: RawStmt::Connect(
                            Expr::Ref("out".to_string()),
                            Expr::SubField(Box::new(Expr::Ref("add".to_string())), "c".to_string()),
                        )
                    }
                ])
            }
        ],
    };
     */

    let r = Circuit {
        pos: None,
        id: "Circuit114514".to_string(),
        modules: vec![Module {
            pos: None,
            // is_gen: false,
            id: "Add".to_string(),
            ports: vec![
                Port {
                    pos: None,
                    dir: Dir::Input,
                    bind: TypeBind("a".to_string(), Type::Uint(32)),
                },
                Port {
                    pos: None,
                    dir: Dir::Input,
                    bind: TypeBind("b".to_string(), Type::Uint(32)),
                },
                Port {
                    pos: None,
                    dir: Dir::Output,
                    bind: TypeBind("c".to_string(), Type::Uint(32)),
                },
            ],
            stmts: StmtGroup(vec![Stmt {
                pos: None,
                raw_stmt: RawStmt::Node(
                    "c".to_string(),
                    Expr::Primop(
                        Primop::Add,
                        vec![Expr::Ref("a".to_string()), Expr::Ref("b".to_string())],
                    ),
                ),
            }]),
        }],
    };

    let pm = r.type_check(());
    println!("analysis:\n{:?}", pm);

    let pm = pm.unwrap();


    let out = r.gen_verilog(&pm);

    println!("{}", out);
}
