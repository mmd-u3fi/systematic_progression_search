use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Graph {
    nodes: HashSet<u32>,
    edges: HashMap<u32, HashSet<u32>>,
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
                }
            }
        }
        Graph {
            nodes,
            edges: edges,
        }
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
            self.edges.iter()
            .filter(|(_, v)| v.contains(&id))
            .map(|(k, _)| *k)
        )
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
    fn unconstrained_nodes() {
        let nodes: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let g = Graph::new(nodes, orderings);
        let unconstrained = g.get_unconstrained_nodes();
        assert_eq!(unconstrained, HashSet::from([1, 2]));
    }

    #[test]
    fn incoming_edges() {
        let nodes: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        let g = Graph::new(nodes, orderings);
        let result = g.get_incoming_edges(3);
        assert_eq!(result, HashSet::from([1, 2]))
    }
}
