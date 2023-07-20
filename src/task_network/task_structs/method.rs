use std::hash::Hash;

use crate::task_network::network::HTN;

#[derive(Debug)]
pub struct Method <'a, T: Hash + Eq>{
    pub name: String,
    pub decomposition: HTN<'a, T>
}

impl <'a, T: Hash + Eq> Method <'a, T> {
    pub fn new(name: String, decomposition: HTN<'a, T>) -> Method<'a, T> {
        Method { name: name, decomposition: decomposition }
    }
}