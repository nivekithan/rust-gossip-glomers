use std::collections::HashMap;

pub struct Topology {
    node_id: Option<String>,
    topology: Option<HashMap<String, Vec<String>>>,
}

impl Topology {
    pub fn new() -> Self {
        return Topology {
            topology: None,
            node_id: None,
        };
    }

    pub fn set_topology(&mut self, topology: HashMap<String, Vec<String>>) {
        self.topology = Some(topology);
    }

    pub fn set_id(&mut self, id: String) {
        self.node_id = Some(id);
    }

    pub fn get(&self) -> &Vec<String> {
        let topology = self.topology.as_ref().unwrap();
        let id = self.node_id.as_ref().unwrap();

        let node_topology = topology.get(id).unwrap();

        return node_topology;
    }
}
