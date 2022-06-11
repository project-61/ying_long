use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::ylir::Id;

lazy_static! {
    pub static ref UNNAME_ID: Mutex<usize> = Mutex::new(0);
}

pub fn gen_id() -> Id {
    let mut id = UNNAME_ID.lock().unwrap();
    *id += 1;
    format!("__gen_id_{}__", id)
}
