use std::collections::HashMap;

use crate::node::NODE;

pub struct Topology {
    topology: Option<HashMap<String, Vec<String>>>,
}

impl Topology {
    pub fn new() -> Self {
        return Topology { topology: None };
    }

    pub fn set_topology(&mut self, topology: HashMap<String, Vec<String>>) {
        self.topology = Some(topology);
    }

    pub fn get(&self) -> &Vec<String> {
        let topology = self.topology.as_ref().unwrap();
        let id: &str = NODE.get().unwrap().id.as_ref();

        let node_topology = topology.get(id).unwrap();

        return node_topology;
    }
}
