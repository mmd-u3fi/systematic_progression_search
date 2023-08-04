use super::*;
use std::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;
use std::collections::HashSet;

fn create_initial_tasks() -> (Rc<Task<u32>>, Rc<Task<u32>>, Rc<Task<u32>>, Rc<Task<u32>>) {
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
        let (t5, t6, t7, t8, t9) = decomposition_tasks();
        let t3 = Task::Compound(CompoundTask::new("Construct".to_string(), Vec::from([
            Method::new(
                "method-01".to_string(),
                HTN::new(
                    HashSet::from([1, 2, 3, 4, 5]),
                    Vec::from([(1, 2), (2, 3), (2, 4), (3, 5), (4, 5)]),
                    HashMap::from(
                        [(1, Rc::new(t5)), (2, Rc::new(t6)), (3, Rc::new(t7)), (4, Rc::new(t8)), (5, Rc::new(t9))]
                    ),
                ),
            )
        ])));
        let t4 = Task::Primitive(PrimitiveAction::new(
            "PayBuilder".to_string(),
            empty.clone(),
            empty.clone(),
            empty.clone(),
        ));
        let (t1, t2, t3, t4) = (Rc::new(t1), Rc::new(t2), Rc::new(t3), Rc::new(t4));
        (t1, t2, t3, t4)
    }

    fn decomposition_tasks() -> (
        Task<u32>,
        Task<u32>,
        Task<u32>,
        Task<u32>,
        Task<u32>,
    ) {
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

    pub fn create_problem_instance() -> HTN<u32>{
        let t: HashSet<u32> = HashSet::from([1, 2, 3, 4]);
        let (t1, t2, t3, t4) = create_initial_tasks();
        let alpha = HashMap::from([(1, t1), (2, t2), (3, t3), (4, t4)]);
        let orderings: Vec<(u32, u32)> = Vec::from([(1, 3), (2, 3), (3, 4)]);
        HTN::new(t, orderings, alpha)
    }