use std::collections::HashMap;

use crate::{ylir::{type_system::Type, *}, utils::gen_id};


pub struct GlobalEnv();


pub trait TypeCheck<T> {
    type Target;
    fn type_check(&self, env: T) -> Result<Self::Target, ()>;

}

pub trait TypeInference<T> {
    fn type_infer(&self, env: T) -> Option<Type>;
}


impl TypeCheck<()> for Circuit {
    type Target = Circuit;
    fn type_check(&self, _: ()) -> Result<Self::Target, ()> {
        let mut old_r = self.clone();
        let mut r = old_r.clone();
        for i in &self.modules {
            // if r.is_some() {
                old_r = r.clone();
                r = i.type_check(r)?;
            // } else {
                // return Ok(old_r);
            // }
        }
        Ok(r)
    }
}

impl TypeCheck<Circuit> for Module {
    type Target = Circuit;

    fn type_check(&self, env: Circuit) -> Result<Self::Target, ()> {
        let ty_env = self.ports.type_check(())?;
        let r = self.stmts.type_check((ty_env, env));
        r.map(|(ty_env, sg, env)| {
            todo!("is not implment type environment reapply to module");
            env
        })
    }
}

impl TypeCheck<()> for Ports {
    type Target = HashMap<Id, Type>;

    fn type_check(&self, _: ()) -> Result<Self::Target, ()> {
        self.iter().map(|x| x.type_check(())).collect()
    }
}
impl TypeCheck<()> for Port {
    type Target = (Id, Type);

    fn type_check(&self, _: ()) -> Result<Self::Target, ()> {
        Ok((self.bind.0.clone(), self.bind.1.clone()))
    }
}

impl TypeCheck<(HashMap<Id, Type>, Circuit)> for StmtGroup {
    type Target = (HashMap<Id, Type>, Self, Circuit);

    fn type_check(&self, env: (HashMap<Id, Type>, Circuit)) -> Result<Self::Target, ()> {
        let mut old_r = env;
        let mut r = old_r.clone();
        let mut sg = vec![];
        for i in self.0.iter() {
            // if r.1.is_some() {
                old_r = r.clone();
                let (ty_env, stmt, top) =
                    i.type_check(r)?;
                sg.push(stmt);
                r = (ty_env, top);
            // } else {
                // return Ok((old_r.0, StmtGroup(sg), old_r.1));
            // }
        }
        Ok((r.0, StmtGroup(sg), r.1))
    }
}

impl TypeCheck<(HashMap<Id, Type>, Circuit)> for Stmt {
    type Target = (HashMap<Id, Type>, Self, Circuit);

    fn type_check(&self, mut env: (HashMap<Id, Type>, Circuit)) -> Result<Self::Target, ()> {
        let mut this = self.clone();
        match &mut this.raw_stmt {
            RawStmt::WireDef(bind) => {
                env.0.insert(bind.0.clone(), bind.1.clone());
                Ok((env.0, this, env.1))
            },
            RawStmt::RegDef(bind, e, _append) => todo!(),
            RawStmt::MemDef(mem) => todo!(),
            RawStmt::Inst(module_name, inst_name) => {
                let mut modu = env.1.modules
                    .iter()
                    .find(|x| &x.id == module_name)
                    .expect(format!("module {} not found", module_name).as_str())
                    .clone();

                let rt = module_type_infer(&modu)?;
                env.0.extend(rt);

                if !modu.is_gen && modu.is_uninstenced() {
                    let new_id = format!("{}$${}", modu.id, gen_id());
                    *module_name = new_id.clone();

                    modu.id = new_id;
                    modu.is_gen = true;
                    env.1.modules.push(modu);
                }

                Ok((env.0, this, env.1))
            },
            RawStmt::Node(id, expr) => {
                if let Some(x) = expr.type_check((&env.0, env.0.get(id).cloned()))? {
                    env.0.insert(id.clone(), x);
                }
                Ok((env.0, this, env.1))
            },
            RawStmt::Connect(expr_left, expr_right) => todo!(),
            RawStmt::When(when) => todo!(),
            RawStmt::StmtGroup(sg) => {
                let (a, b, c) = sg.type_check(env)?;
                Ok((a, Stmt {
                    pos: self.pos.clone(),
                    raw_stmt: RawStmt::StmtGroup(b),
                }, c))
            },
        }
    }
}


#[inline]
fn module_type_infer(this: &Module) -> Result<HashMap<Id, Type>, ()> {
    this.ports.type_check(())
}

/*
impl TypeInference<()> for Module {
    fn type_infer(&self, _: ()) -> Option<Type> {
        self.ports.type_check(())
    }
}
 */

 impl TypeCheck<(&HashMap<Id, Type>, Option<Type>)> for Expr {
    type Target = Option<Type>;

    fn type_check(&self, env: (&HashMap<Id, Type>, Option<Type>)) -> Result<Self::Target, ()> {
        match self {
            Expr::Literal(_) => todo!(),
            Expr::Ref(_) => todo!(),
            Expr::SubField(_, _) => todo!(),
            Expr::SubIndex(_, _) => todo!(),
            Expr::SubAccess(_, _) => todo!(),
            Expr::Mux(_, _, _) => todo!(),
            Expr::Primop(_, _) => todo!(),
        }
    }
}