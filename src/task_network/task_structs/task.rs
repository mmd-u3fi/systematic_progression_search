use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};

use super::{PrimitiveAction, CompoundTask};

#[derive(Debug)]
pub enum Task <'a, T> where T: Eq + Hash {
    Primitive(PrimitiveAction<T>),
    Compound(CompoundTask<'a, T>),
}

impl <'a, T: Eq + Hash> PartialEq for Task<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Primitive(x) => {
                match other {
                    Self::Primitive(y) => {
                        x.name == y.name
                    }
                    Self::Compound(_) => false
                }
            }
            Self::Compound(x) => {
                match other {
                    Self::Primitive(_) => false,
                    Self::Compound(y) => {
                        x.name == y.name
                    }
                }
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(&other)
    }
}

impl <'a, T: Eq + Hash> Eq for Task<'a, T> {}

impl <'a, T: Eq + Hash> Hash for Task<'a, T> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self {
            Task::Compound(x) => {
                x.name.hash(hasher)
            }
            Task::Primitive(x) => {
                x.name.hash(hasher)
            }
        }
    }
}