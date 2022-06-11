use crate::ylir::*;

use super::Pass;

#[derive(Debug, Default)]
pub struct SimFilter();

impl Pass<&mut SimFilter> for Circuit {
    fn pass(&mut self, pm: &mut SimFilter) {
        for i in self.modules.iter_mut() {
            i.pass(pm);
        }
    }
}

impl Pass<&mut SimFilter> for Module {
    fn pass(&mut self, _pm: &mut SimFilter) {
        let r = self.stmts.0.iter().filter(filter).cloned().collect();
        self.stmts.0 = r;
    }
}

fn filter(stmt: &&Stmt) -> bool {
    matches!(
        stmt.raw_stmt,
        RawStmt::WireDef(_)             |
        RawStmt::RegDef(_, _, _)        |
        RawStmt::MemDef(_)              |
        RawStmt::Inst(_, _)             |
        RawStmt::Node(_, _)             |
        RawStmt::Connect(_, _)          |
        // RawStmt::PartialConnect(_, _)   |
        RawStmt::When(_)                |
        // RawStmt::Invalidate(_)          |
        // RawStmt::Stop(_)                |
        // RawStmt::Printf(_)              |
        // RawStmt::Skip                   |
        RawStmt::StmtGroup(_)
    )
}
