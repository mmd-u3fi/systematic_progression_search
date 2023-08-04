use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: HashSet<u32>,
    pub edges: HashMap<u32, HashSet<u32>>,
}

impl Graph {
    pub fn new(nodes: HashSet<u32>, orderings: Vec<(u32, u32)>) -> Self {
        let mut edges: HashMap<u32, HashSet<u32>> = HashMap::with_capacity(nodes.len());
        for edge in orderings.into_iter() {
            match edge {
                (x, y) => match edges.get_mut(&x) {
                    Some(val) => {
                        val.insert(y);
                    }
                    None => {
                        edges.insert(x, HashSet::from([y]));
                    }
                },
            }
        }
        Graph {
            nodes,
            edges: edges,
        }
    }

    pub fn get_edges(&self) -> Vec<(u32, u32)> {
        self.edges
            .clone()
            .into_iter()
            .map(|(k, v)| repeat(k).zip(v).collect::<Vec<_>>())
            .flatten()
            .collect()
    }

    pub fn convert_edges_to_vec(edges: &HashMap<u32, HashSet<u32>>) -> Vec<(u32, u32)> {
        edges
            .clone()
            .into_iter()
            .map(|(k, v)| repeat(k).zip(v).collect::<Vec<_>>())
            .flatten()
            .collect()
    }

    pub fn count_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_neighbors(&self, id: u32) -> Option<&HashSet<u32>> {
        self.edges.get(&id)
    }

    pub fn get_unconstrained_nodes(&self) -> HashSet<u32> {
        let mut result = self.nodes.clone();
        for k in self.edges.keys() {
            for val in self.edges.get(k).unwrap() {
                result.remove(val);
            }
        }
        result
    }

    pub fn get_incoming_edges(&self, id: u32) -> HashSet<u32> {
        HashSet::from_iter(
            self.edges
                .iter()
                .filter(|(_, v)| v.contains(&id))
                .map(|(k, _)| *k),
        )
    }

    pub fn remove_node(&self, id: u32) -> Graph {
        if !self.nodes.contains(&id) {
            self.clone()
        } else {
            let mut new_nodes = self.nodes.clone();
            new_nodes.remove(&id);
            let mut new_edges = Vec::with_capacity(self.edges.keys().len());
            for (key, values) in self.edges.iter() {
                if *key == id {
                    continue;
                }
                for value in values {
                    if *value == id {
                        continue;
                    } else {
                        new_edges.push((*key, *value))
                    }
                }
            }
            Graph::new(new_nodes, new_edges)
        }
    }

    pub fn add_subgraph(
        &self,
        subgraph: Graph,
        incoming_edges: HashSet<u32>,
        outgoing_edges: HashSet<u32>,
    ) -> Graph {
        let nodes = self.nodes.clone().union(&subgraph.nodes).cloned().collect();
        let mut orderings = self.edges.clone();

        // Adding incoming edges
        let unconstrained_nodes = subgraph.get_unconstrained_nodes();
        for node in incoming_edges.iter() {
            match orderings.contains_key(node) {
                false => orderings.insert(*node, unconstrained_nodes.clone()),
                true => orderings.insert(
                    *node,
                    unconstrained_nodes
                        .union(orderings.get(node).unwrap())
                        .cloned()
                        .collect(),
                ),
            };
        }

        // Adding outgoing edges
        let terminal_nodes: HashSet<u32> = subgraph
            .nodes
            .difference(&subgraph.edges.keys().cloned().collect())
            .cloned()
            .collect();
        for node in terminal_nodes.iter() {
            orderings.insert(*node, outgoing_edges.clone());
        }

        for (key, value) in subgraph.edges {
            orderings.insert(key, value);
        }

        let orderings = orderings
            .into_iter()
            .map(|(k, v)| repeat(k).zip(v).collect::<Vec<_>>())
            .flatten()
            .collect();
        Graph::new(nodes, orderings)
    }

    pub fn to_layers(&self) -> Vec<HashSet<u32>> {
        let mut result: Vec<HashSet<u32>> = Vec::new();
        let mut prev_layer = self.get_unconstrained_nodes();
        result.push(prev_layer.clone());
        loop {
            let mut layer: HashSet<u32> = HashSet::new();
            for node in prev_layer.iter() {
                match self.edges.get(node) {
                    Some(x) => {
                        for outgoing in x.iter() {
                            layer.insert(*outgoing);
                        }
                    }
                    None => continue,
                }
            }
            if layer.is_empty() {
                break;
            }
            result.push(layer.clone());
            prev_layer = layer;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn instantiation() {
        let nodes: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let g = Graph::new(nodes, orderings);
        assert_eq!(g.count_nodes(), 4);
        assert!(g.get_neighbors(1).unwrap().contains(&3));
        assert!(g.get_neighbors(2).unwrap().contains(&3));
        assert!(g.get_neighbors(3).unwrap().contains(&4));
    }

    #[test]
    fn unconstrained_nodes_test() {
        let nodes: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let g = Graph::new(nodes, orderings);
        let unconstrained = g.get_unconstrained_nodes();
        assert_eq!(unconstrained, HashSet::from([1, 2]));
    }

    #[test]
    fn incoming_edges_test() {
        let nodes: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let g = Graph::new(nodes, orderings);
        let result = g.get_incoming_edges(3);
        assert_eq!(result, HashSet::from([1, 2]))
    }

    #[test]
    fn delete_node_test() {
        let nodes: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let g = Graph::new(nodes, orderings);
        let new_g = g.remove_node(3);
        let unconstrainted = new_g.get_unconstrained_nodes();
        assert_eq!(new_g.count_nodes(), 3);
        assert_eq!(unconstrainted, HashSet::from([1, 2, 4]))
    }

    #[test]
    fn add_subgraph_test() {
        let nodes: HashSet<u32> = HashSet::from([1, 2, 4]);
        let orderings: Vec<(u32, u32)> = Vec::from([]);
        let g = Graph::new(nodes, orderings);

        let subgraph_nodes = HashSet::from([5, 6, 7, 8, 9]);
        let subgraph_orderings: Vec<(u32, u32)> =
            Vec::from([(5, 6), (6, 7), (6, 8), (7, 9), (8, 9)]);
        let subgraph = Graph::new(subgraph_nodes, subgraph_orderings);

        let result = g.add_subgraph(subgraph, HashSet::from([1, 2]), HashSet::from([4]));

        // inherited orderings
        assert_eq!(*result.edges.get(&1).unwrap(), HashSet::from([5]));
        assert_eq!(*result.edges.get(&2).unwrap(), HashSet::from([5]));
        assert_eq!(*result.edges.get(&9).unwrap(), HashSet::from([4]));

        //subgraph orderings
        assert_eq!(*result.edges.get(&5).unwrap(), HashSet::from([6]));
        assert_eq!(*result.edges.get(&6).unwrap(), HashSet::from([7, 8]));
        assert_eq!(*result.edges.get(&7).unwrap(), HashSet::from([9]));
        assert_eq!(*result.edges.get(&8).unwrap(), HashSet::from([9]));
    }

    #[test]
    pub fn graph_to_layers_test() {
        // first graph
        let nodes: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let g = Graph::new(nodes, orderings);
        let result = g.to_layers();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], HashSet::from([1, 2]));
        assert_eq!(result[1], HashSet::from([3]));
        assert_eq!(result[2], HashSet::from([4]));
    }
}
