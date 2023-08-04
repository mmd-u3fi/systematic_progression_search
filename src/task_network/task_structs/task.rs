use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};

use super::{CompoundTask, PrimitiveAction};

#[derive(Debug)]
pub enum Task<T>
where
    T: Eq + Hash,
{
    Primitive(PrimitiveAction<T>),
    Compound(CompoundTask<T>),
}

impl<T: Eq + Hash> PartialEq for Task<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Primitive(x) => match other {
                Self::Primitive(y) => x.name == y.name,
                Self::Compound(_) => false,
            },
            Self::Compound(x) => match other {
                Self::Primitive(_) => false,
                Self::Compound(y) => x.name == y.name,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(&other)
    }
}

impl<T: Eq + Hash> Eq for Task<T> {}

impl<T: Eq + Hash> Hash for Task<T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            Task::Compound(x) => x.name.hash(hasher),
            Task::Primitive(x) => x.name.hash(hasher),
        }
    }
}

impl <T:Hash + Eq> Task<T> {
    pub fn get_name(&self) -> String {
        match self {
            Task::Compound(CompoundTask { name, .. }) => name.clone(),
            Task::Primitive(PrimitiveAction { name, .. }) => name.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::task_network::{CompoundTask, PrimitiveAction};

    use super::Task;
    #[test]
    pub fn task_name_test() {
        let compound = Task::Compound(CompoundTask::<u32>::new("task1".to_string(), Vec::new()));
        assert_eq!(compound.get_name(), "task1".to_string());

        let primitive = Task::Primitive(PrimitiveAction::<u32>::new(
            "task2".to_string(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new()
        ));
        assert_eq!(primitive.get_name(), "task2".to_string());
    }
}