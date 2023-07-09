use std::cmp::PartialEq;
use super::{PrimitiveAction, CompoundTask};

#[derive(Debug)]
pub enum Task <'a> {
    Primitive(PrimitiveAction),
    Compound(CompoundTask<'a>),
}

impl <'a> PartialEq for Task<'a> {
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