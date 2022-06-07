use core::panic;
use std::{collections::HashMap, env::args, ops::AddAssign};

use crate::ylir::*;
use crate::ylir::type_system::*;

use super::{Pass, PurePass, AnaPass};



#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct GlobalEnv (pub HashMap<Id, ModuleEnv>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Port,
    Wire,
    Reg,
    Inst,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModuleEnv (pub HashMap<Id, (NodeType, Type)>);


impl AnaPass<&mut GlobalEnv> for Circuit {
    fn ana_pass(&self, pm: &mut GlobalEnv) {
        self.modules.iter().for_each(|i| i.ana_pass(pm));
    }
}

impl AnaPass<&mut GlobalEnv> for Module {
    fn ana_pass(&self, pm: &mut GlobalEnv) {
        let mut old_m_pm = if pm.0.contains_key(&self.id) {
            pm.0.get(&self.id).unwrap().clone()
        } else {
            ModuleEnv(HashMap::new())
        };
        let mut new_m_pm = old_m_pm.clone();

        self.ports.ana_pass(&mut new_m_pm);

        // self.stmts.ana_pass((&mut new_m_pm, pm));
        // /*
        // find fixed point
        while new_m_pm != old_m_pm {
            old_m_pm = new_m_pm.clone();
            self.stmts.ana_pass((&mut new_m_pm, pm));
        }
        //  */
        pm.0.insert(self.id.clone(), new_m_pm);
    }
}

impl AnaPass<&mut ModuleEnv> for Ports {
    fn ana_pass(&self, pm: &mut ModuleEnv) {
        for i in self.iter() {
            let i = &i.bind;
            pm.0.insert(i.0.clone(), (NodeType::Port, i.1.clone()));
        }
    }
}

impl AnaPass<(&mut ModuleEnv, &mut GlobalEnv)> for StmtGroup {
    fn ana_pass(&self, (a, b): (&mut ModuleEnv, &mut GlobalEnv)) {
        for i in self.0.iter() {
            i.ana_pass((a, b))
        }
        // self.0.iter().for_each(|i| i.ana_pass(pm));
    }
}


/* impl<'a> AnaPass<(&'a mut GlobalEnv, Id)> for StmtGroup {
    fn ana_pass(&self, pm: (&'a mut GlobalEnv, Id)) {
        let m_pm = pm.0.0.get_mut(&pm.1).unwrap();
        // self.0.iter_mut().for_each(|i| i.pass(pm));
        todo!()
    }
} */

impl AnaPass<(&mut ModuleEnv, &mut GlobalEnv)> for Stmt {
    fn ana_pass(&self, (pm, g_pm): (&mut ModuleEnv, &mut GlobalEnv)) {
        // dbg!(&pm);
        match &self.raw_stmt {
            RawStmt::WireDef(s) => { pm.0.insert(s.0.clone(), (NodeType::Wire, s.1.clone())); },
            RawStmt::RegDef(s, _, _) => { pm.0.insert(s.0.clone(), (NodeType::Reg, s.1.clone())); },
            RawStmt::MemDef(s) => { pm.0.insert(s.id.clone(), (NodeType::Reg, s.data_type.clone())); },
            RawStmt::Inst(_, _) => todo!(),
            RawStmt::Node(s, a) => {
                let infer_type = a.pure_pass(Inference(pm));
                if let (Some((nt, raw_ty)), Some(ty)) = (pm.0.get(s), infer_type) {
                    if let Some(ty) = raw_ty.unify(&ty) {
                        pm.0.insert(s.clone(), (nt.clone(), ty));
                    }
                }
            },
            RawStmt::Connect(a, b) => {
                let a_ty = a.pure_pass(Inference(pm));
                let b_ty = b.pure_pass(Inference(pm));
                if let Some(false) = a_ty.map(|a| b_ty.map(|b| a == b)).flatten() {
                    panic!("type mismatch");
                }
            },
            RawStmt::When(s) => {when_apply_module_env(s.as_ref(), (pm, g_pm));},
            RawStmt::StmtGroup(sg) => sg.ana_pass((pm, g_pm)),
        };
    }
}

fn when_apply_module_env(s: &When, pm: (&mut ModuleEnv, &mut GlobalEnv)) -> Option<()> {
    s.then.ana_pass((pm.0, pm.1));
    s.else_.as_ref().map(|x| x.ana_pass((pm.0, pm.1)));
    let cond_ty = s.cond.pure_pass(Inference(pm.0))?;
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

#[derive(Debug, Clone, Copy)]
struct Inference<'a>(pub &'a ModuleEnv);

impl<'a> PurePass<Inference<'a>> for Module {
    type Target = Bundle;

    fn pure_pass(&self, pm: Inference<'a>) -> Self::Target {
        todo!()
    }
}

impl<'a> PurePass<Inference<'a>> for Expr {
    type Target = Option<Type>;

    fn pure_pass(&self, pm: Inference<'a>) -> Self::Target {
        match self {
            Expr::Literal(l) => Some(l.tp.clone()),
            Expr::Ref(id) => id.pure_pass(pm),
            Expr::SubField(id, sf) => {
                let r_type = id.pure_pass(pm).map(|x| x.get_bundle().cloned()).flatten()?;
                let r = r_type.get_field(sf);
                if r.is_none() {
                    panic!("{:?} has no field {}", id, sf);
                    // return None;
                }
                Some(r.unwrap().bind.1.clone())
            },
            Expr::SubIndex(id, si) => {
                let r_type = id.pure_pass(pm).map(|x| x.get_vector().cloned()).flatten()?;
                Some(*r_type.0)
            },
            Expr::SubAccess(id, sa) => {
                // id.pure_pass(pm).map(|x| todo!())
                let r_type = id.pure_pass(pm).map(|x| x.get_vector().cloned()).flatten()?;
                // todo: unify sa type
                Some(*r_type.0)
            },
            Expr::Mux(cond, then, else_) => {
                let cond_tp = cond.pure_pass(pm)?;
                if cond_tp.get_width()? != 1 {
                    panic!("mux condition must be a bit");
                    // return None;
                }
                let then_tp = then.pure_pass(pm)?;
                let else_tp = else_.pure_pass(pm)?;
                if then_tp != else_tp {
                    panic!("mux then and else must be the same type");
                    // return None;
                }
                Some(then_tp)
            },
            Expr::Primop(_op, args) => {
                let mut rt = None;
                for arg in args {
                    let t = arg.pure_pass(pm)?;
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


impl<'a> PurePass<Inference<'a>> for Id {
    type Target = Option<Type>;

    fn pure_pass(&self, pm: Inference<'a>) -> Self::Target {
        pm.0.0.get(self).map(|x| x.1.clone())
    }
}
