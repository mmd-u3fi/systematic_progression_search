pub struct PrimitiveAction {
    name: String
}

impl PrimitiveAction {
    pub fn new(name: String) -> Self {
        PrimitiveAction { name: name }
    }
}