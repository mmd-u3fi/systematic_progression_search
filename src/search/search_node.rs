use std::{collections::HashSet, hash::Hash};
use super::HTN;

pub struct SearchNode<T: Hash + Eq> {
    pub state: HashSet<u32>,
    pub network: HTN<T>,
    pub sequence: Vec<String>
}

impl <T:Hash + Eq> SearchNode<T> {
    pub fn is_goal(&self) -> bool {
        unimplemented!()
    }
}