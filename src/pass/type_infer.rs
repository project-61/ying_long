use std::{collections::HashMap, env::args};

use crate::ylir::*;
use crate::ylir::type_system::*;

use super::{Pass, PurePass};



#[derive(Debug, Clone)]
pub struct GlobalEnv (pub HashMap<Id, ModuleEnv>);

#[derive(Debug, Clone)]
pub enum NodeType {
    Port,
    Wire,
    Reg,
    Inst,
}

#[derive(Debug, Clone)]
pub struct ModuleEnv (pub HashMap<Id, (NodeType, Type)>);


impl Pass<GlobalEnv> for Circuit {
    fn pass(&mut self, pm: &mut GlobalEnv) {
        self.modules.iter_mut().for_each(|i| i.pass(pm));
    }
}

impl Pass<GlobalEnv> for Module {
    fn pass(&mut self, pm: &mut GlobalEnv) {
        if pm.0.contains_key(&self.id) {
            let m_pm = pm.0.get_mut(&self.id).unwrap();
            self.stmts.pass(m_pm);
        } else {
            let mut m_pm = ModuleEnv(HashMap::new());
            self.stmts.pass(&mut m_pm);
            pm.0.insert(self.id.clone(), m_pm);
        }
    }
}

impl Pass<ModuleEnv> for StmtGroup {
    fn pass(&mut self, pm: &mut ModuleEnv) {
        self.0.iter_mut().for_each(|i| i.pass(pm));
    }
}

impl Pass<ModuleEnv> for Stmt {
    fn pass(&mut self, pm: &mut ModuleEnv) {
        match &self.raw_stmt {
            RawStmt::WireDef(s) => pm.0.insert(s.0.clone(), (NodeType::Wire, s.1.clone())),
            RawStmt::RegDef(s, _, _) => pm.0.insert(s.0.clone(), (NodeType::Reg, s.1.clone())),
            RawStmt::MemDef(s) => pm.0.insert(s.id.clone(), (NodeType::Reg, s.data_type.clone())),
            RawStmt::Inst(_, _) => todo!(),
            RawStmt::Node(s, a) => todo!(),
            RawStmt::Connect(_, _) => todo!(),
            RawStmt::When(_) => todo!(),
            RawStmt::StmtGroup(_) => todo!(),
        };
    }
}

#[derive(Debug, Clone)]
struct Inference<'a>(pub &'a ModuleEnv);

impl<'a> PurePass<Inference<'a>> for Module {
    type Target = Bundle;

    fn pure_pass(&self, pm: &Inference) -> Self::Target {
        todo!()
    }
}


/*
impl PurePass<Inference> for StmtGroup {
    type Target = Option<Type>;

    fn pure_pass(&self, pm: &Inference) -> Self::Target {

    }
}
*/


impl<'a> PurePass<Inference<'a>> for Stmt {
    type Target = Option<Type>;

    fn pure_pass(&self, pm: &Inference) -> Self::Target {
        match &self.raw_stmt {
            RawStmt::WireDef(_) => None,
            RawStmt::RegDef(_, _, _) => None,
            RawStmt::MemDef(_) => None,
            RawStmt::Inst(_, _) => None,
            RawStmt::Node(_, expr) => todo!(),
            RawStmt::Connect(a, b) => todo!(),
            RawStmt::When(_) => None,
            RawStmt::StmtGroup(_) => None,
        }
    }
}

impl<'a> PurePass<Inference<'a>> for Expr {
    type Target = Option<Type>;

    fn pure_pass(&self, pm: &Inference) -> Self::Target {
        match self {
            Expr::Literal(l) => Some(l.tp.clone()),
            Expr::Ref(id) => id.pure_pass(pm),
            Expr::SubField(id, sf) => {
                let r_type = id.pure_pass(pm).map(|x| x.get_bundle()).flatten()?;
                let r = r_type.get_field(sf);
                if r.is_none() {
                    // panic!("{} has no field {}", id, sf);
                    return None;
                }
                let r = r.unwrap().bind.1;
                Some(r)
            },
            Expr::SubIndex(id, si) => {
                let r_type = id.pure_pass(pm).map(|x| x.get_vector()).flatten()?;
                Some(r_type.0.as_ref().clone())
            },
            Expr::SubAccess(id, sa) => {
                // id.pure_pass(pm).map(|x| todo!())
                let r_type = id.pure_pass(pm).map(|x| x.get_vector()).flatten()?;
                // todo: unify sa type
                Some(r_type.0.as_ref().clone())
            },
            Expr::Mux(cond, then, else_) => {
                let cond_tp = cond.pure_pass(pm)?;
                if cond_tp.get_width()? != 1 {
                    // todo: error message here (mux condition must be a bit)
                    return None;
                }
                let then_tp = then.pure_pass(pm)?;
                let else_tp = else_.pure_pass(pm)?;
                if then_tp != else_tp {
                    // todo: error message here (mux then and else must be the same type)
                    return None;
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

    fn pure_pass(&self, pm: &Inference) -> Self::Target {
        pm.0.0.get(self).map(|x| x.1.clone())
    }
}
