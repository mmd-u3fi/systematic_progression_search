use std::collections::{HashMap, HashSet};
use super::graph::Graph;
use super::task_structs::{Task, PrimitiveAction, CompoundTask};

#[derive(Debug)]
pub struct HTN <'a> {
    network: Graph,
    mappings: HashMap<u32, &'a Task<'a>>,
}

impl <'a> HTN <'a>{
    pub fn new(tasks: HashSet<u32>, orderings: Vec<(u32, u32)>, mappings: HashMap<u32, &'a Task>) -> HTN<'a> {
        HTN {
            network: Graph::new(tasks, orderings),
            mappings
        }
    }

    pub fn count_tasks(&self) -> usize {
        self.network.count_nodes()
    }

    pub fn get_task(&self, id: u32) -> Option<&Task> {
        match self.mappings.get_key_value(&id) {
            Some((_, y)) => Some(*y),
            None => None
        }
    }

    pub fn get_unconstrained_tasks(&self) -> HashSet<u32> {
        self.network.get_unconstrained_nodes()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_initial_tasks<'a>() -> (Task<'a>, Task<'a>, Task<'a>, Task<'a>) {
        let t1 = Task::Primitive(PrimitiveAction::new("ObtainPermit".to_string()));
        let t2 = Task::Primitive(PrimitiveAction::new("HireBuilder".to_string()));
        let t3 = Task::Compound(CompoundTask::new("Construct".to_string(), Vec::new()));
        let t4 = Task::Primitive(PrimitiveAction::new("PayBuilder".to_string()));
        (t1, t2, t3, t4)
    }

    #[test]
    fn instantiation() {
        let t: HashSet<u32> = HashSet::from([1,2,3,4]);
        let (t1, t2, t3, t4) = create_initial_tasks();
        let alpha: HashMap<u32, &Task> = HashMap::from(
            [(1, &t1), (2, &t2), (3, &t3), (4, &t4)]
        );
        let orderings: Vec<(u32, u32)> = Vec::from(
            [(1,3), (2,3), (3,4)]
        );
        let network = HTN::new(t, orderings, alpha);
        assert_eq!(network.count_tasks(), 4);
        assert_eq!(network.get_task(1).unwrap(), &t1);
        assert_eq!(network.get_task(2).unwrap(), &t2);
        assert_eq!(network.get_task(3).unwrap(), &t3);
        assert_eq!(network.get_task(4).unwrap(), &t4);
        assert_eq!(network.get_task(5), None);
    }

    #[test]
    fn unconstrained_tasks() {
        let t: HashSet<u32> = HashSet::from([1,2,3,4]);
        let (t1, t2, t3, t4) = create_initial_tasks();
        let alpha: HashMap<u32, &Task> = HashMap::from(
            [(1, &t1), (2, &t2), (3, &t3), (4, &t4)]
        );
        let orderings: Vec<(u32, u32)> = Vec::from(
            [(1,3), (2,3), (3,4)]
        );
        let network = HTN::new(t, orderings, alpha);
        let unconstrained = network.get_unconstrained_tasks();
        assert_eq!(unconstrained, HashSet::from([1,2]));
    }
}