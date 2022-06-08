use core::panic;
use std::{collections::HashMap};

use crate::ylir::*;
use crate::ylir::type_system::*;

// use super::{Pure, PurePass, StatePass};



#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct GlobalEnv (pub HashMap<Id, ModuleEnv>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Port(Dir),
    Wire,
    Reg,
    Inst,
}

impl NodeType {
    pub fn get_port(&self) -> Option<Dir> {
        match self {
            NodeType::Port(dir) => Some(dir.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModuleEnv (pub HashMap<Id, (NodeType, Type)>);

pub trait TypeCheck<T> {
    fn type_check(&mut self, env: T);

}

pub trait TypeInference<T> {
    fn type_infer(&self, env: T) -> Option<Type>;
}

pub trait TypeSet {
    fn set_type(&mut self, env: (&mut ModuleEnv, &mut GlobalEnv), ty: &Type) -> Option<()>;
}


impl TypeCheck<&mut GlobalEnv> for Circuit {
    fn type_check(&mut self, pm: &mut GlobalEnv) {
        for i in self.modules.iter_mut() {
            i.type_check(pm);
        }
        // .for_each(|i| i.state_pass(pm));
    }
}

impl TypeCheck<&mut GlobalEnv> for Module {
    fn type_check(&mut self, pm: &mut GlobalEnv) {
        let mut old_m_pm = if pm.0.contains_key(&self.id) {
            pm.0.get(&self.id).unwrap().clone()
        } else {
            ModuleEnv(HashMap::new())
        };
        let mut new_m_pm = old_m_pm.clone();

        self.ports.type_check(&mut new_m_pm);

        // self.stmts.ana_state_pass((&mut new_m_pm, pm));
        // /*
        // find fixed point
        while new_m_pm != old_m_pm {
            old_m_pm = new_m_pm.clone();
            self.stmts.type_check((&mut new_m_pm, pm));
        }
        //  */
        pm.0.insert(self.id.clone(), new_m_pm);
    }
}

impl TypeCheck<&mut ModuleEnv> for Ports {
    fn type_check(&mut self, pm: &mut ModuleEnv) {
        for i in self.iter() {
            let ity = &i.bind;
            pm.0.insert(ity.0.clone(), (NodeType::Port(i.dir.clone()), ity.1.clone()));
        }
    }
}

impl TypeCheck<(&mut ModuleEnv, &mut GlobalEnv)> for StmtGroup {
    fn type_check(&mut self, (a, b): (&mut ModuleEnv, &mut GlobalEnv)) {
        for i in self.0.iter_mut() {
            i.type_check((a, b));
        }
        // self.0.iter().for_each(|i| i.ana_state_pass(pm));
    }
}


/* impl<'a> AnaStatePass<(&'a mut GlobalEnv, Id)> for StmtGroup {
    fn ana_state_pass(&mut self, pm: (&'a mut GlobalEnv, Id)) {
        let m_pm = pm.0.0.get_mut(&pm.1).unwrap();
        // self.0.iter_mut().for_each(|i| i.state_pass(pm));
        todo!()
    }
} */

impl TypeCheck<(&mut ModuleEnv, &mut GlobalEnv)> for Stmt {
    fn type_check(&mut self, (pm, g_pm): (&mut ModuleEnv, &mut GlobalEnv)) {
        // dbg!(&pm);
        match &mut self.raw_stmt {
            RawStmt::WireDef(s) => { pm.0.insert(s.0.clone(), (NodeType::Wire, s.1.clone())); },
            RawStmt::RegDef(s, _, _) => { pm.0.insert(s.0.clone(), (NodeType::Reg, s.1.clone())); },
            RawStmt::MemDef(s) => { pm.0.insert(s.id.clone(), (NodeType::Reg, s.data_type.clone())); },
            RawStmt::Inst(module_name, inst_name) => {
                let m_pm = g_pm.0.get(module_name).unwrap().clone();

                todo!()
            },
            RawStmt::Node(s, a) => {
                let infer_type = a.type_infer((pm, g_pm));
                if let (Some((nt, raw_ty)), Some(ty)) = (pm.0.get(s), infer_type) {
                    if let Some(ty) = raw_ty.unify(&ty) {
                        pm.0.insert(s.clone(), (nt.clone(), ty));
                    }
                }
            },
            RawStmt::Connect(a, b) => {
                let a_ty = a.type_infer((pm, g_pm));
                let b_ty = b.type_infer((pm, g_pm));
                let rt = a_ty.map(|a| b_ty.map(|b| a.unify(&b))).flatten().flatten();
                if rt.is_none() {
                    panic!("type mismatch");
                }
                let rt = rt.unwrap();
            },
            RawStmt::When(s) => {when_apply_module_env(s.as_mut(), (pm, g_pm));},
            RawStmt::StmtGroup(sg) => sg.type_check((pm, g_pm)),
        };
    }
}

fn when_apply_module_env(s: &mut When, pm: (&mut ModuleEnv, &mut GlobalEnv)) -> Option<()> {
    s.then.type_check((pm.0, pm.1));
    s.else_.as_mut().map(|x| x.type_check((pm.0, pm.1)));
    let cond_ty = s.cond.type_infer(pm)?;
    if cond_ty
        .get_width()
        .map(|x| if x != 1 { Some(()) } else {None})
        .flatten()
        .is_some() {
        // todo: panic
        return None;
    }
    Some(())
}

// /*
impl<'a> TypeInference<&Id> for ModuleEnv {
    fn type_infer(&self, id: &Id) -> Option<Type> {
        let pm = self.0.get(id)?;
        let r = self.0
            .iter()
            .filter(|(_, (nt, _))| matches!(nt, NodeType::Port(_)))
            .map(|(id, (nt, ty))| {
                let nt = nt.get_port().unwrap();
                Field {
                    is_flip: matches!(nt, Dir::Input),
                    bind: TypeBind(id.clone(), ty.clone()),
                }
            })
            .collect();
        Some(Type::Bundle(Bundle(r)))
    }
}
//  */

impl TypeInference<(&mut ModuleEnv, &mut GlobalEnv)> for Expr {
    fn type_infer(&self, (pa, pb): (&mut ModuleEnv, &mut GlobalEnv)) -> Option<Type> {
        match self {
            Expr::Literal(l) => Some(l.tp.clone()),
            Expr::Ref(id) => id.type_infer((pa, pb)),
            Expr::SubField(id, sf) => {
                let r_type = id.type_infer((pa, pb)).map(|x| x.get_bundle().cloned()).flatten()?;
                let r = r_type.get_field(sf);
                if r.is_none() {
                    panic!("{:?} has no field {}", id, sf);
                    // return None;
                }
                Some(r.unwrap().bind.1.clone())
            },
            Expr::SubIndex(id, _si) => {
                let r_type = id.type_infer((pa, pb)).map(|x| x.get_vector().cloned()).flatten()?;
                Some(*r_type.0)
            },
            Expr::SubAccess(id, _sa) => {
                // id.pure_state_pass(pm).map(|x| todo!())
                let r_type = id.type_infer((pa, pb)).map(|x| x.get_vector().cloned()).flatten()?;
                // todo: unify sa type
                Some(*r_type.0)
            },
            Expr::Mux(cond, then, else_) => {
                let cond_tp = cond.type_infer((pa, pb))?;
                if cond_tp.get_width()? != 1 {
                    panic!("mux condition must be a bit");
                    // return None;
                }
                let then_tp = then.type_infer((pa, pb))?;
                let else_tp = else_.type_infer((pa, pb))?;
                if then_tp != else_tp {
                    panic!("mux then and else must be the same type");
                    // return None;
                }
                Some(then_tp)
            },
            Expr::Primop(_op, args) => {
                let mut rt = None;
                for arg in args {
                    let t = arg.type_infer((pa, pb))?;
                    if rt.is_none() {
                        rt = Some(t);
                    } else {
                        if rt.as_ref().unwrap() != &t {
                            return None;
                        }
                    }
                }
                rt
            },
        }
    }
}

impl TypeSet for Expr {
    fn set_type(&mut self, (pm, pg): (&mut ModuleEnv, &mut GlobalEnv), ty: &Type) -> Option<()> {
        match self {
            Expr::Literal(l) => None,
            Expr::Ref(id) => {
                let (nt, _) = pm.0.get(id).unwrap();
                pm.0.insert(id.clone(), (nt.clone(), ty.clone()));
                Some(())
            },
            Expr::SubField(id, sf) => todo!(),
            Expr::SubIndex(expr, _) => {
                // expr.type_infer((pm, pg));
                todo!()
            }
            Expr::SubAccess(expr, _) => {
                expr.type_infer((pm, pg));
                // let (nt, ty) = pm.0.get(id).unwrap();

                // pm.0.insert(id.clone(), (nt.clone(), ty.clone()));
                Some(())
            },
            Expr::Mux(_c, t, e) => {
                t.set_type((pm, pg), ty)?;
                e.set_type((pm, pg), ty)
            },
            Expr::Primop(_op, args) =>
                args.iter_mut().try_for_each(|x| x.set_type((pm, pg), ty)),
        }
    }
}

impl TypeInference<(&mut ModuleEnv, &mut GlobalEnv)> for Id {
    fn type_infer(&self, (pm, g_pm): (&mut ModuleEnv, &mut GlobalEnv)) -> Option<Type> {
        if let r@Some(_) = pm.0.get(self).map(|x| x.1.clone()) {
            return r;
        }
        g_pm.0.get(self).map(|x| x.type_infer(self)).flatten()
    }
}

impl TypeSet for Id {
    fn set_type(&mut self, env: (&mut ModuleEnv, &mut GlobalEnv), ty: &Type) -> Option<()> {
        todo!()
    }
}