use std::{collections::HashMap, sync::{Arc, Mutex, atomic::AtomicU64}};

use lazy_static::*;

pub type Handle<T> = Arc<T>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(Handle<String>);

impl Symbol {
    pub fn new() -> Self {
        Symbol(string_intern(&format!("gen_{}", get_new_id())))
    }
}

impl From<&str> for Symbol {
    fn from(i: &str) -> Self {
        Symbol(string_intern(i))
    }
}

lazy_static! {
    static ref GLOBAL_NUMBER: AtomicU64 = AtomicU64::new(0);
    static ref GLOBAL_INTERN_STRING_POOL: Mutex<HashMap<Handle<String>, Handle<String>>> =
        Mutex::new(HashMap::new());
}

#[inline]
fn get_new_id() -> u64 {
    GLOBAL_NUMBER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

#[inline]
fn string_intern(i: &str) -> Handle<String> {
    let k = Handle::new(i.to_string());
    {
        if let Some(x) = GLOBAL_INTERN_STRING_POOL.lock().unwrap().get(&k) {
            return x.clone();
        }
    }
    GLOBAL_INTERN_STRING_POOL
        .lock()
        .unwrap()
        .insert(k.clone(), k.clone());
    k
}