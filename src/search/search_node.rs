use std::{collections::HashSet, hash::Hash};
use super::HTN;

pub struct SearchNode<T: Hash + Eq> {
    state: HashSet<u32>,
    network: HTN<T>,
    sequence: Vec<String>
}