use super::{PrimitiveAction, CompoundTask};
pub enum Task {
    Primitive(PrimitiveAction),
    Compound(CompoundTask),
}