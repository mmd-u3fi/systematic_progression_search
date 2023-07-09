#[derive(Debug, PartialEq)]
pub struct PrimitiveAction {
    pub name: String
}

impl PrimitiveAction {
    pub fn new(name: String) -> Self {
        PrimitiveAction { name: name }
    }
}