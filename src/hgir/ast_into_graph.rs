use super::*;

impl Into<graph::Module> for ast::Module {
    fn into(self) -> graph::Module {
        let name = &self.name;
        let input = &self.input;
        let output = &self.output;
        // self.items.into_iter().map(|x| x.into()).collect();
        todo!()
    }
}

impl Into<i32> for ast::Hardware {
    fn into(self) -> i32 {
        todo!()
    }
}