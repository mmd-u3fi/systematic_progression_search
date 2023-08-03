use std::hash::Hash;

use super::Method;

#[derive(Debug)]
pub struct CompoundTask<'a, T: Hash + Eq> {
    pub name: String,
    methods: Vec<Method<'a, T>>,
}

impl<'a, T: Hash + Eq> CompoundTask<'a, T> {
    pub fn new(name: String, methods: Vec<Method<'a, T>>) -> Self {
        CompoundTask { name, methods }
    }

    pub fn add_method(&mut self, method: Method<'a, T>) {
        self.methods.push(method);
    }
}
