use crate::task_network::network::HTN;

#[derive(Debug)]
pub struct Method <'a>{
    pub name: String,
    pub decomposition: HTN<'a>
}