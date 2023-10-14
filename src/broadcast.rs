use std::collections::HashSet;

use crate::{
    message::{Message, MessageBody},
    node::NODE,
    topology::Topology,
};

pub struct BroadcastService {
    pub topology: Topology,
    messages: HashSet<usize>,
}

impl BroadcastService {
    pub fn new() -> Self {
        return BroadcastService {
            topology: Topology::new(),
            messages: HashSet::new(),
        };
    }

    pub fn add_message(&mut self, new_message: usize) {
        if !self.messages.contains(&new_message) {
            self.broadcast(new_message);
            self.messages.insert(new_message);
        }
    }

    pub fn get_message(&self) -> &HashSet<usize> {
        return &self.messages;
    }

    pub fn broadcast(&self, message: usize) {
        let current_node_id = &NODE.get().unwrap().id;
        self.topology.get().iter().for_each(|other_node_id| {
            let message = Message::new(
                current_node_id.clone(),
                other_node_id.clone(),
                MessageBody::node_broadcast { message },
            );

            message.send();
        })
    }
}
