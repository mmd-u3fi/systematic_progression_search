use super::Method;

#[derive(Debug)]
pub struct CompoundTask <'a>{
    pub name: String,
    methods: Vec<Method<'a>>
}

impl <'a> CompoundTask <'a>{
    pub fn new(name: String, methods: Vec<Method<'a>>) -> Self {
        CompoundTask {
            name,
            methods
        }
    }

    pub fn add_method(&mut self, method: Method<'a>) {
        self.methods.push(method);
    }
}