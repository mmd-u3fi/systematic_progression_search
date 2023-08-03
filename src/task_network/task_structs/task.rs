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
