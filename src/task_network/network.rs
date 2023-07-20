use super::graph::Graph;
use super::task_structs::{CompoundTask, Method, PrimitiveAction, Task};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub struct HTN<'a, T: Hash + Eq> {
    network: Graph,
    mappings: HashMap<u32, &'a Task<'a, T>>,
}

impl<'a, T: Hash + Eq> HTN<'a, T> {
    pub fn new(
        tasks: HashSet<u32>,
        orderings: Vec<(u32, u32)>,
        mappings: HashMap<u32, &'a Task<T>>,
    ) -> HTN<'a, T> {
        HTN {
            network: Graph::new(tasks, orderings),
            mappings,
        }
    }

    pub fn count_tasks(&self) -> usize {
        self.network.count_nodes()
    }

    pub fn get_task(&self, id: u32) -> Option<&Task<T>> {
        match self.mappings.get_key_value(&id) {
            Some((_, y)) => Some(*y),
            None => None,
        }
    }

    pub fn get_unconstrained_tasks(&self) -> HashSet<u32> {
        self.network.get_unconstrained_nodes()
    }

    pub fn get_incoming_edges(&self, id: u32) -> HashSet<u32> {
        self.network.get_incoming_edges(id)
    }

    pub fn decompose(&self, id: u32, method: &'a Method<T>) -> HTN<'a, T> {
        // TODO: Refactor this function
        let mut subgraph_nodes = method.decomposition.network.nodes.clone();
        let mut subgraph_edges = method.decomposition.network.edges.clone();
        let mut subgraph_mappings = method.decomposition.mappings.clone();
        if !subgraph_nodes.is_disjoint(&self.network.nodes) {
            let intersection: HashSet<u32> =
                method.decomposition.mappings.keys().cloned().collect();
            let network_max_id = self.network.nodes.iter().fold(u32::MIN, |a, b| a.max(*b));
            let max_id = subgraph_nodes.iter().fold(network_max_id, |a, b| a.max(*b));
            let new_ids: HashMap<u32, u32> = intersection.into_iter().zip(max_id + 1..).collect();
            for (prev_id, new_id) in new_ids.iter() {
                let mapping_val = subgraph_mappings.remove(&prev_id).unwrap();
                subgraph_mappings.insert(*new_id, mapping_val);
                if subgraph_edges.contains_key(&prev_id) {
                    let edges: HashSet<u32> = subgraph_edges.remove(&prev_id).unwrap();
                    let mapped_edges = edges.into_iter().map(|x| {
                        if new_ids.contains_key(&x) {
                            *new_ids.get(&x).unwrap()
                        } else {
                            x
                        }
                    });

                    subgraph_edges.insert(*new_id, mapped_edges.collect());
                }
                subgraph_nodes.remove(&prev_id);
                subgraph_nodes.insert(*new_id);
            }
        }
        let mut new_graph = self.network.clone();
        let outgoing_edges = self.network.edges.get(&id).unwrap().clone();
        let incoming_edges = self.network.get_incoming_edges(id);

        new_graph = new_graph.remove_node(id);
        new_graph = new_graph.add_subgraph(
            Graph::new(
                subgraph_nodes.clone(),
                Graph::convert_edges_to_vec(&subgraph_edges),
            ),
            incoming_edges,
            outgoing_edges,
        );

        let mut new_mappings = self.mappings.clone();
        new_mappings.remove(&id);
        for (id, m) in subgraph_mappings {
            new_mappings.insert(id, m);
        }
        let new_nodes = self
            .network
            .nodes
            .iter()
            .filter(|x| **x != id)
            .cloned()
            .collect::<HashSet<u32>>()
            .union(&subgraph_nodes)
            .cloned()
            .collect();
        HTN::new(new_nodes, new_graph.get_edges(), new_mappings)
    }

    pub fn is_isomorphic(tn1: &HTN<'a, T>, tn2: &HTN<'a, T>) -> bool {
        let layers_1 = tn1.network.to_layers();
        let layers_2 = tn2.network.to_layers();
        if layers_1.len() != layers_2.len() {
            return false;
        }
        let tasks_1 = tn1.layers_to_tasks(layers_1);
        let tasks_2 = tn2.layers_to_tasks(layers_2);

        for (x, y) in tasks_1.into_iter().zip(tasks_2.into_iter()) {
            if x != y {
                return false;
            }
        }

        return true;
    }

    fn layers_to_tasks(&self, layers: Vec<HashSet<u32>>) -> Vec<HashSet<&'a Task<'a, T>>> {
        let mut result = Vec::with_capacity(layers.len());
        for layer in layers.into_iter() {
            let tasks = layer.into_iter().map(|x| *self.mappings.get(&x).unwrap());
            result.push(tasks.collect());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_initial_tasks<'a>() -> (Task<'a, u32>, Task<'a, u32>, Task<'a, u32>, Task<'a, u32>) {
        let empty = HashSet::new();
        let t1 = Task::Primitive(PrimitiveAction::new(
            "ObtainPermit".to_string(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
        ));
        let t2 = Task::Primitive(PrimitiveAction::new(
            "HireBuilder".to_string(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
        ));
        let t3 = Task::Compound(CompoundTask::new("Construct".to_string(), Vec::new()));
        let t4 = Task::Primitive(PrimitiveAction::new(
            "PayBuilder".to_string(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
        ));
        (t1, t2, t3, t4)
    }

    #[test]
    fn instantiation() {
        let t: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let (t1, t2, t3, t4) = create_initial_tasks();
        let alpha: HashMap<u32, &Task<u32>> = HashMap::from([(1, &t1), (2, &t2), (3, &t3), (4, &t4)]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let network = HTN::new(t, orderings, alpha);
        assert_eq!(network.count_tasks(), 4);
        assert_eq!(network.get_task(1).unwrap(), &t1);
        assert_eq!(network.get_task(2).unwrap(), &t2);
        assert_eq!(network.get_task(3).unwrap(), &t3);
        assert_eq!(network.get_task(4).unwrap(), &t4);
        assert_eq!(network.get_task(5), None);
    }

    fn decomposition_tasks<'a>() -> (Task<'a, u32>, Task<'a, u32>, Task<'a, u32>, Task<'a, u32>, Task<'a, u32>) {
        let empty = HashSet::new();
        let t1 = Task::Primitive(PrimitiveAction::new(
            "BuildFoundation".to_string(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
        ));
        let t2 = Task::Primitive(PrimitiveAction::new(
            "BuildFrame".to_string(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
        ));
        let t3 = Task::Primitive(PrimitiveAction::new(
            "BuildRoof".to_string(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
        ));
        let t4 = Task::Primitive(PrimitiveAction::new(
            "BuildWalls".to_string(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
        ));
        let t5 = Task::Primitive(PrimitiveAction::new(
            "BuildInterior".to_string(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
        ));
        (t1, t2, t3, t4, t5)
    }

    #[test]
    fn unconstrained_tasks_test() {
        let t: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let (t1, t2, t3, t4) = create_initial_tasks();
        let alpha: HashMap<u32, &Task<u32>> = HashMap::from([(1, &t1), (2, &t2), (3, &t3), (4, &t4)]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let network = HTN::new(t, orderings, alpha);
        let unconstrained = network.get_unconstrained_tasks();
        assert_eq!(unconstrained, HashSet::from([1, 2]));
    }

    #[test]
    fn decomposition_test() {
        let t: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let (t1, t2, t3, t4) = create_initial_tasks();
        let (t5, t6, t7, t8, t9) = decomposition_tasks();
        let t3_method = Method::new(
            "method-01".to_string(),
            HTN::new(
                HashSet::from([1, 2, 3, 4, 5]),
                Vec::from([(1, 2), (2, 3), (2, 4), (3, 5), (4, 5)]),
                HashMap::from([(1, &t5), (2, &t6), (3, &t7), (4, &t8), (5, &t9)]),
            ),
        );
        let alpha: HashMap<u32, &Task<u32>> = HashMap::from([(1, &t1), (2, &t2), (3, &t3), (4, &t4)]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let network = HTN::new(t, orderings, alpha);
        let result = network.decompose(3, &t3_method);
        assert_eq!(result.count_tasks(), 8);
        assert_eq!(result.get_unconstrained_tasks(), HashSet::from([1, 2]));
        assert_eq!(Graph::convert_edges_to_vec(&result.network.edges).len(), 8);
        assert_eq!(result.network.get_incoming_edges(1).len(), 1);
    }

    #[test]
    pub fn isomorphism_test() {
        let (t1, t2, t3, t4) = create_initial_tasks();
        // first graph
        let nodes1: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let orderings1: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let htn1 = HTN::new(
            nodes1,
            orderings1,
            HashMap::from([(1, &t1), (2, &t2), (3, &t3), (4, &t4)]),
        );

        let (t5, t6, t7, t8) = create_initial_tasks();
        // second graph
        let nodes2: HashSet<u32> = HashSet::from([5, 6, 7, 8]);
        let orderings2: Vec<(u32, u32)> = Vec::from([(5, 7), (6, 7), (7, 8)]);
        let htn2 = HTN::new(
            nodes2,
            orderings2,
            HashMap::from([(5, &t5), (6, &t6), (7, &t7), (8, &t8)]),
        );

        let result = HTN::is_isomorphic(&htn1, &htn2);
        assert_eq!(result, true);
    }
}
