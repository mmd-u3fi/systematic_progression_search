use std::collections::HashMap;
use super::task_structs::{Task, PrimitiveAction, CompoundTask};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn instantiation() {
        let t: Vec<u32> = Vec::from([1,2,3,4]);
        // TODO: introduce a struct to own, and keep track of the tasks
        let t1 = Task::Primitive(PrimitiveAction::new("ObtainPermit".to_string()));
        let t2 = Task::Primitive(PrimitiveAction::new("HireBuilder".to_string()));
        let t3 = Task::Compound(CompoundTask::new("Construct".to_string()));
        let t4 = Task::Primitive(PrimitiveAction::new("PayBuilder".to_string()));
        let alpha: HashMap<u32, &Task> = HashMap::from(
            [(1, &t1), (2, &t2), (3, &t3), (4, &t4)]
        );
        let ordering: Vec<(u32, u32)> = Vec::from(
            [(1,3), (2,3), (3,4)]
        );
        let network = HTN::new(t, alpha, ordering);
        assert_eq!(network.count_nodes(), 4);
        assert_eq!(network.count_edges(), 3);
    }
}