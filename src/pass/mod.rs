pub mod gen_verilog;
pub mod sim_filter;
pub mod type_infer;

pub trait Pass<PM> {
    fn pass(&mut self, pm: PM);
}

/*
pub trait StatePass<PM> {
    fn state_pass(&mut self, pm: &mut PM);
}
 */

pub trait PurePass<PM> {
    type Target;
    fn pure_pass(&self, pm: PM) -> Self::Target;
}
