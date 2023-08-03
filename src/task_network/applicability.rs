use std::{collections::HashSet, hash::Hash};

pub trait Applicability<T: Eq + Hash> {
    fn is_applicable(&self, state: &HashSet<T>) -> bool;
}
