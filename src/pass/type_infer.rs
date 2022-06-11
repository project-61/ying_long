use std::collections::HashMap;

use crate::ylir::{
    type_system::{Type, TypeBind},
    *,
};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct GlobalEnv(pub HashMap<Id, Module>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Port(Dir),
    Wire,
    Reg,
    Inst,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModuleEnv(pub HashMap<Id, (Dir, TypeBind)>);

pub trait TypeCheck<T> {
    type Target;

    fn type_check(&self, env: T) -> Result<Self::Target, ()>;
}

pub trait TypeInference<T> {
    type Target;

    fn type_infer(&self, env: T) -> Self::Target;
}

impl TypeCheck<()> for Circuit {
    type Target = GlobalEnv;
    fn type_check(&self, _: ()) -> Result<Self::Target, ()> {
        // let mut old_r = ;
        let genv: HashMap<Id, Module> = self
            .modules
            .iter()
            .map(|modu| (modu.id.clone(), modu.clone()))
            .collect();
        let genv = GlobalEnv(genv);
        for i in &self.modules {
            i.type_check(&genv)?;
        }
        Ok(genv)
    }
}

impl TypeCheck<&GlobalEnv> for Module {
    type Target = ();

    fn type_check(&self, genv: &GlobalEnv) -> Result<Self::Target, ()> {
        let mut env = ModuleEnv(
            self.ports
                .type_infer(())
                .into_iter()
                .map(|x| (x.0, (-x.1 .0, x.1 .1)))
                .collect(),
        );
        for i in self.stmts.0.iter().filter(|x| {
            matches!(
                x.raw_stmt,
                RawStmt::WireDef(_) | RawStmt::RegDef(_, _, _) | RawStmt::MemDef(_)
            )
        }) {
            match &i.raw_stmt {
                RawStmt::WireDef(bind) => {
                    env.0.insert(bind.0.clone(), (Dir::Inout, bind.clone()));
                }
                RawStmt::RegDef(bind, cls, rst) => {
                    env.0.insert(bind.0.clone(), (Dir::Output, bind.clone()));
                    todo!()
                }
                RawStmt::MemDef(mem) => {
                    env.0.insert(
                        mem.id.clone(),
                        (Dir::Output, TypeBind(mem.id.clone(), mem.data_type.clone())),
                    );
                    // todo
                }
                RawStmt::Inst(mod_name, inst_name) => {
                    let modu = genv
                        .0
                        .get(mod_name)
                        .expect(&format!("module is not defined: {}", mod_name));
                    let r = modu.ports.type_infer(());
                }
                _ => unreachable!(),
            }
        }
        self.stmts.type_check((env, &genv)); // fixme
        Ok(())
    }
}

impl TypeInference<()> for Ports {
    type Target = HashMap<Id, (Dir, TypeBind)>;

    fn type_infer(&self, _: ()) -> Self::Target {
        self.iter()
            .map(|x| {
                let r = x.type_infer(());
                (x.bind.0.clone(), r)
            })
            .collect()
    }
}

impl TypeInference<()> for Port {
    type Target = (Dir, TypeBind);

    fn type_infer(&self, _: ()) -> Self::Target {
        (self.dir, self.bind.clone())
    }
}

impl TypeCheck<(ModuleEnv, &GlobalEnv)> for StmtGroup {
    type Target = ModuleEnv;

    fn type_check(&self, (mut env, genv): (ModuleEnv, &GlobalEnv)) -> Result<Self::Target, ()> {
        for i in self.0.iter() {
            env = i.type_check((env, genv))?;
        }
        Ok(env)
    }
}

impl TypeCheck<(ModuleEnv, &GlobalEnv)> for Stmt {
    type Target = ModuleEnv;

    fn type_check(&self, (mut env, genv): (ModuleEnv, &GlobalEnv)) -> Result<Self::Target, ()> {
        match &self.raw_stmt {
            RawStmt::Node(left, right) => {
                let lt = env.0.get(left).unwrap();
                let rt = env.0.get(left).unwrap(); // fixme

                todo!()
            }
            RawStmt::Connect(a, b) => {
                todo!()
            }
            RawStmt::When(when) => todo!(),
            RawStmt::StmtGroup(_) => todo!(),
            _ => Ok(env),
        }
    }
}

impl TypeInference<&ModuleEnv> for Expr {
    type Target = (Option<Dir>, Type);

    fn type_infer(&self, env: &ModuleEnv) -> Self::Target {
        match self {
            Expr::Literal(a) => (Some(Dir::Input), a.typ.clone()),
            Expr::Ref(id) => {
                let r = env
                    .0
                    .get(id)
                    .expect(&format!("name is not defined: {}", id));
                (Some(r.0), r.1 .1.clone())
            },
            Expr::SubField(e, sf) => {
                let (dir, ty) = e.type_infer(env);

                let ty = ty.get_bundle().expect("subfield expr is not bundle");
                let r = ty.0.get(sf).unwrap();
                // (r.is_flip, r.1.clone())
                todo!()
            },
            Expr::SubIndex(e, si) => e.type_infer(env),
            Expr::SubAccess(e, sa) => {
                let (dir, left_ty) = e.type_infer(env);

                let (dir, right_ty) = sa.type_infer(env);
                // assert!(dir.is_output(), "expr is not output");


                todo!()
            }
            Expr::Mux(c, t, e) => todo!(),
            Expr::Primop(_, args) => todo!(),
        }
    }
}

impl TypeInference<&ModuleEnv> for Id {
    type Target = Result<(Dir, TypeBind), ()>;

    fn type_infer(&self, env: &ModuleEnv) -> Self::Target {
        env.0.get(self).map_or(Err(()), |x| Ok(x.clone()))
    }
}
