use std::{collections::HashSet, hash::Hash};

pub trait Applicability<T: Eq + Hash + Clone> {
    fn is_applicable(&self, state: &HashSet<T>) -> bool;
    fn transition(&self, state: &HashSet<T>) -> HashSet<T>;
}
