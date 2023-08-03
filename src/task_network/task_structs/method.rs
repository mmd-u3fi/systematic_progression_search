use std::hash::Hash;

use crate::task_network::network::HTN;

#[derive(Debug)]
pub struct Method<T: Hash + Eq> {
    pub name: String,
    pub decomposition: HTN<T>,
}

impl<T: Hash + Eq> Method<T> {
    pub fn new(name: String, decomposition: HTN<T>) -> Method<T> {
        Method {
            name: name,
            decomposition: decomposition,
        }
    }
}
