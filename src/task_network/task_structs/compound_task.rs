use std::hash::Hash;

use super::Method;

#[derive(Debug)]
pub struct CompoundTask<T: Hash + Eq> {
    pub name: String,
    methods: Vec<Method<T>>,
}

impl<T: Hash + Eq> CompoundTask<T> {
    pub fn new(name: String, methods: Vec<Method<T>>) -> Self {
        CompoundTask { name, methods }
    }

    pub fn add_method(&mut self, method: Method<T>) {
        self.methods.push(method);
    }

    pub fn methods_iter(&self) -> impl Iterator<Item = &Method<T>> {
        self.methods.iter()
    }
}
