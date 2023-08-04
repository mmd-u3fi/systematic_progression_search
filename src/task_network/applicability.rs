use std::{collections::HashSet};

pub trait Applicability {
    type T;
    fn is_applicable(&self, state: &HashSet<Self::T>) -> bool;
    fn transition(&self, state: &HashSet<Self::T>) -> HashSet<Self::T>
    where Self::T: Clone;
}
