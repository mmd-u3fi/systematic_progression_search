use std::collections::HashSet;
use std::hash::Hash;

use super::HTN;
use super::search_result::SearchResult;
use super::search_node::SearchNode;
use super::fringe::Fringe;

pub struct ProgressionSearch<U> 
where U: Fringe {
    fringe: U
}

impl <U: Fringe> ProgressionSearch<U> {
    pub fn new(fringe: U) -> ProgressionSearch<U> {
        ProgressionSearch { fringe }
    }

    pub fn run<T:Hash + Eq>(initial_state: HashSet<u32>, initial_network: HTN<T>) -> SearchResult {
        unimplemented!()
    }

    pub fn expand_node<T:Hash + Eq>(node: SearchNode<T>) -> Vec<SearchNode<T>> {
        unimplemented!()
    }
}