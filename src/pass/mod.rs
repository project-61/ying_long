pub mod sim_filter;
pub mod type_infer;
pub mod gen_verilog;


pub trait Pass<PM> {
    fn pass(&mut self, pm: PM);
}

pub trait AnaPass<PM> {
    fn ana_pass(&self, pm: PM);
}

pub trait PurePass<PM> {
    type Target;
    fn pure_pass(&self, pm: PM) -> Self::Target;
}