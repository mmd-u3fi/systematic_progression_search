use std::{collections::HashSet, hash::Hash};
use super::HTN;

pub struct SearchNode<T: Hash + Eq> {
    pub state: HashSet<T>,
    pub network: HTN<T>,
    pub sequence: Vec<String>
}

impl <T: Hash + Eq> SearchNode<T> {
    pub fn new(state: HashSet<T>, network: HTN<T>, sequence: Vec<String>) -> SearchNode<T> {
        SearchNode { state, network, sequence }
    }

    pub fn is_goal(&self) -> bool {
        self.network.count_tasks() == 0
    }
}