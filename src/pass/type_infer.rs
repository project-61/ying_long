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
        // let mut old_r = ;
        let mut r = self.clone();
        for i in &self.modules {
            // if r.is_some() {
                // old_r = r.clone();
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
        let mut this = self.clone();
        let ty_env = self.ports.type_check(())?;
        let (rt, sg, mut root) = self.stmts.type_check((ty_env, env))?;
        this.stmts = sg;
        apply_rt_for_module(&mut this, dbg!(&rt));
        let modu = root.modules.iter_mut().find(|modu| modu.id == self.id).unwrap();
        *modu = this;
        Ok(root)
    }
}

fn apply_rt_for_module(modu: &mut Module, rt: &HashMap<Id, Type>) {
    for Stmt { raw_stmt, .. } in modu.stmts.0.iter_mut() {
        match raw_stmt {
            RawStmt::WireDef(bind) |
            RawStmt::RegDef(bind, _, _) => {
                if let Some(rt) = rt.get(&bind.0) {
                    bind.1 = rt.clone();
                }
            }
            RawStmt::MemDef(Mem { id, data_type, ..}) => {
                if let Some(rt) = rt.get(id) {
                    *data_type = rt.clone();
                }
            }
            _ => return,
        }
    }
    for i in modu.ports.iter_mut() {
        if let Some(rt) = rt.get(&i.bind.0) {
            i.bind.1 = rt.clone();
        }
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
        // let mut old_r = env;
        let mut r = env;
        let mut sg = vec![];
        for i in self.0.iter() {
            // if r.1.is_some() {
                // old_r = r.clone();
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
                /*
                match expr.type_check((&env.0, env.0.get(id).cloned()))? {
                    (Some(x), env, e) => {
                        env.0.insert(id.clone(), x);
                        *expr = e;
                    },
                    (None, e) => {
                        *expr = e;
                    }
                }
                 */
                let (_, re, e) = expr.type_check((&env.0, env.0.get(id).cloned()))?;
                *expr = e;
                Ok((re, this, env.1))
            },
            RawStmt::Connect(expr_left, expr_right) => {
                let (right_type, re, e) = expr_right.type_check((&env.0, None))?;
                *expr_right = e;
                let (_left_type, re, e) = expr_left.type_check((&re, right_type))?;
                *expr_left = e;
                Ok((re, this, env.1))
            },
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
    type Target = (Option<Type>, HashMap<Id, Type>, Self);

    fn type_check(&self, env: (&HashMap<Id, Type>, Option<Type>)) -> Result<Self::Target, ()> {
        let mut this = self.clone();
        match &mut this {
            Expr::Literal(li) => {
                let rt = if let Some(x) = env.1 {
                    li.tp.unify(&x)
                } else {
                    Some(li.tp.clone())
                };
                Ok((rt, env.0.clone(), this))
            },
            Expr::Ref(id) =>
                if let Ok(rt) = id.type_check((env.0, env.1)) { Ok((rt, env.0.clone(), this)) } else { Err(()) },
            Expr::SubField(parent, sf) => {
                /*
                if Some(src_type) = env.1 {

                }
                let (rt, env, _) = parent.type_check((env.0, None))?;

                rt
                 */
                todo!()
            },
            Expr::SubIndex(parent, si) => todo!(),
            Expr::SubAccess(parent, sa) => todo!(),
            Expr::Mux(c, t, e) => {
                let (_, env, rc) = c.type_check(
                    (env.0, Some(Type::Uint(Some(1)))))?;
                *c = Box::new(rc);
                let (rt, env, te) = t.type_check((&env, None))?;
                *t = Box::new(te);
                let (rt, env, ee) = t.type_check((&env, None))?;
                *e = Box::new(ee);
                todo!()
            },
            Expr::Primop(_op, args) => {
                let mut unify_type = env.1;
                let mut ret_args = vec![];
                let mut env = env.0.clone();
                for i in args.iter() {
                    let (rt, renv, e) = i.type_check((&env, unify_type))?;
                    env = renv;
                    unify_type = rt;
                    ret_args.push(e);
                }
                *args = ret_args;
                Ok((unify_type, env, this))
            },
        }
    }
}

impl TypeCheck<(&HashMap<Id, Type>, Option<Type>)> for Id {
    type Target = Option<Type>;

    fn type_check(&self, env: (&HashMap<Id, Type>, Option<Type>)) -> Result<Self::Target, ()> {
        Ok(env.0.get(self).cloned())
    }
}