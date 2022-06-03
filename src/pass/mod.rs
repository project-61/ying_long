pub mod sim_filter;
pub mod gen_verilog;


pub trait Pass<PM> {
    fn pass(&mut self, pm: &mut PM);
}

pub trait PurePass<PM> {
    type Target;
    fn pure_pass(&self, pm: &PM) -> Self::Target;
}