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

impl<T> Applicability<T> for PrimitiveAction<T>
where
    T: Eq + Hash,
{
    fn is_applicable(&self, state: &HashSet<T>) -> bool
    where
        T: Eq + Hash,
    {
        for condition in self.pre_cond.iter() {
            if !state.contains(condition) {
                return false;
            }
        }
        true
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
}
