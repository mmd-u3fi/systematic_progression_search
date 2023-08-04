use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

use crate::task_network::{Task, PrimitiveAction};

use super::HTN;
use super::search_result::SearchResult;
use super::search_node::SearchNode;
use super::Applicability;

pub struct ProgressionSearch<T: Hash + Eq>{
    fringe: VecDeque<SearchNode<T>>
}

impl <T: Hash + Eq + Clone + std::fmt::Debug> ProgressionSearch<T> {
    pub fn new() -> ProgressionSearch<T> {
        ProgressionSearch { fringe: VecDeque::new() }
    }

    pub fn run(&mut self, initial_state: HashSet<T>, initial_network: HTN<T>) -> SearchResult {
        let init = SearchNode::new(initial_state, initial_network, Vec::new());
        self.fringe.push_back(init);
        while !self.fringe.is_empty() {
            println!("-----------------");
            println!("fringe size: {:?}", self.fringe.len());
            let n = self.fringe.pop_front().unwrap();
            if n.is_goal() { return SearchResult::Solved(n.sequence);}
            let unconstrained = n.network.get_unconstrained_tasks();
            let u_a: HashSet<u32> = unconstrained.iter().filter(|x| n.network.is_primitive(**x)).cloned().collect();
            let u_c: HashSet<u32> = unconstrained.iter().filter(|x| !u_a.contains(*x)).cloned().collect();
            println!("unconstrained: {:?}", unconstrained);
            println!("u_a: {:?}", u_a);
            println!("u_c: {:?}", u_c);
            println!("network size: {:?}", n.network.count_tasks());
            if u_c.is_empty() {
                for t in u_a.iter(){
                    let task = n.network.get_task(*t).unwrap();
                    if let Task::Primitive(a) = task {
                        if a.is_applicable(&n.state) {
                            let new_network = n.network.apply_action(*t);
                            let new_state = a.transition(&n.state);
                            let mut new_sequence = n.sequence.clone();
                            new_sequence.push(task.get_name());
                            let new_search_node = SearchNode::new(
                                new_state,
                                new_network,
                                new_sequence
                            );
                            self.fringe.push_back(new_search_node)
                        }
                    }
                }
            } else {
                let t = u_c.iter().next().unwrap();
                let task = n.network.get_task(*t).unwrap();
                if let Task::Compound(c) = task {
                    for m in c.methods.iter() {
                        println!("decomposing {:?} using {:?}", task.get_name(), m.name);
                        let new_network = n.network.decompose(*t, m);
                        println!("{:?}", new_network);
                        let new_search_node = SearchNode::new(
                            n.state.clone(),
                            new_network,
                            n.sequence.clone()
                        );
                        self.fringe.push_back(new_search_node);
                    }
                }
            }
        }
        SearchResult::Unsolvable
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::example::create_problem_instance;

    // TODO: Fix this test
    #[test]
    pub fn hierarchy_correctness_test() {
        let htn = create_problem_instance();
        let mut search = ProgressionSearch::<u32>::new();
        let result = search.run(HashSet::new(), htn);
        if let SearchResult::Solved(x) = result {
            assert_eq!(x[2], "BuildFoundation");
            assert_eq!(x[3], "BuildFrame");
            assert_eq!(x[6], "BuildInterior");
            assert_eq!(x[7], "PayBuilder");
        }

    }
}