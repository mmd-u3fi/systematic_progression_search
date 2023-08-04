use std::{collections::HashSet, hash::Hash};

use crate::task_network::applicability::Applicability;

#[derive(Debug, PartialEq)]
pub struct PrimitiveAction<T: Eq + Hash> {
    pub name: String,
    pre_cond: HashSet<T>,
    add_effects: HashSet<T>,
    del_effects: HashSet<T>,
}

impl<T: Eq + Hash> PrimitiveAction<T> {
    pub fn new(
        name: String,
        pre_cond: HashSet<T>,
        add_effects: HashSet<T>,
        del_effects: HashSet<T>,
    ) -> Self {
        PrimitiveAction {
            name,
            pre_cond,
            add_effects,
            del_effects,
        }
    }
}

impl <U: Eq + Hash> Applicability for PrimitiveAction<U> {
    type T = U;
    fn is_applicable(&self, state: &HashSet<Self::T>) -> bool
    {
        for condition in self.pre_cond.iter() {
            if !state.contains(condition) {
                return false;
            }
        }
        true
    }

    fn transition(&self, state: &HashSet<Self::T>) -> HashSet<Self::T>
    where Self::T: Eq + Hash + Clone{
        let mut new_state: HashSet<Self::T> = state
            .iter()
            .cloned()
            .filter(|x| !self.del_effects.contains(x))
            .collect();
        for add in self.add_effects.iter() {
            new_state.insert(add.clone());
        }
        new_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn applicability_test() {
        let mut state = HashSet::from(["is_loaded", "object_visible"]);
        let precond = HashSet::from(["is_loaded", "object_visible"]);
        let action = PrimitiveAction::new(
            "Action1".to_string(),
            precond,
            HashSet::from([]),
            HashSet::from([]),
        );
        assert_eq!(action.is_applicable(&state), true);
        state.insert("is_close");
        assert_eq!(action.is_applicable(&state), true);
        state.remove("object_visible");
        assert_eq!(action.is_applicable(&state), false);
    }

    #[test]
    pub fn transition_test() {
        let mut state = HashSet::from(["is_loaded", "object_visible"]);
        let precond = HashSet::from(["is_loaded", "object_visible"]);
        let action = PrimitiveAction::new(
            "Action1".to_string(),
            precond,
            HashSet::from(["ready"]),
            HashSet::from(["is_loaded"]),
        );
        let new_state = action.transition(&state);
        assert_eq!(new_state.contains("ready"), true);
        assert_eq!(new_state.len(), 2);
    }
}
