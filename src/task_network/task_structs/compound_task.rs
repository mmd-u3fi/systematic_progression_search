use std::hash::Hash;

use super::Method;

#[derive(Debug)]
pub struct CompoundTask<T: Hash + Eq> {
    pub name: String,
    pub methods: Vec<Method<T>>,
}

impl<T: Hash + Eq> CompoundTask<T> {
    pub fn new(name: String, methods: Vec<Method<T>>) -> Self {
        CompoundTask { name, methods }
    }

    pub fn add_method(&mut self, method: Method<T>) {
        self.methods.push(method);
    }
}
