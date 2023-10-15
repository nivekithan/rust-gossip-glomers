use std::{
    sync::{Arc, Mutex},
    time::UNIX_EPOCH,
};

use crate::node::Node;

pub struct Counter {
    sequence: Arc<Mutex<usize>>,
}

impl Counter {
    pub fn new() -> Self {
        return Counter {
            sequence: Arc::new(Mutex::new(0)),
        };
    }
    fn current_time() -> u128 {
        return UNIX_EPOCH.elapsed().unwrap().as_millis();
    }

    pub fn generate_unique_msg_id(&self) -> usize {
        let sequence = self.sequence.clone();
        let mut sequence = sequence.lock().unwrap();
        let msg_id = *sequence;

        *sequence += 1;

        return msg_id;
    }
    pub fn generate_unique_id(&self, node: &Node) -> String {
        let now = Self::current_time();
        let node_id = &node.id;
        let sequence = self.generate_unique_msg_id();
        return format!("{now}{node_id}{sequence}");
    }
}
