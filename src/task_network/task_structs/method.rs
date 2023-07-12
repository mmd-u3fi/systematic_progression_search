use crate::task_network::network::HTN;

#[derive(Debug)]
pub struct Method <'a>{
    pub name: String,
    pub decomposition: HTN<'a>
}

impl <'a> Method <'a> {
    pub fn new(name: String, decomposition: HTN<'a>) -> Method<'a> {
        Method { name: name, decomposition: decomposition }
    }
}