use std::time::UNIX_EPOCH;

use crate::node::Node;

pub struct Counter {
    sequence: usize,
}

impl Counter {
    pub fn new() -> Self {
        return Counter { sequence: 0 };
    }
    fn current_time() -> u128 {
        return UNIX_EPOCH.elapsed().unwrap().as_millis();
    }

    pub fn generate_unique_msg_id(&mut self) -> usize {
        let msg_id = self.sequence;

        self.sequence += 1;

        return msg_id;
    }
    pub fn generate_unique_id(&mut self, node: &Node) -> String {
        let now = Self::current_time();
        let node_id = &node.id;
        let sequence = self.sequence;

        self.sequence += 1;
        return format!("{now}{node_id}{sequence}");
    }
}
