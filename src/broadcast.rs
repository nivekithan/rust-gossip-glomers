use std::collections::HashSet;

use crate::{
    counter::Counter,
    message::{Message, MessageBody},
    node::NODE,
    pending_request::PendingRequestService,
    topology::Topology,
};

pub struct BroadcastService {
    messages: HashSet<usize>,
    pub topology: Topology,
    pending_request: PendingRequestService,
    counter: Counter,
}

impl BroadcastService {
    pub fn new() -> Self {
        return BroadcastService {
            topology: Topology::new(),
            messages: HashSet::new(),
            pending_request: PendingRequestService::new(),
            counter: Counter::new(),
        };
    }

    pub async fn add_message(&mut self, new_message: usize) {
        if !self.messages.contains(&new_message) {
            self.messages.insert(new_message);
            self.broadcast(new_message).await;
        }
    }

    pub fn get_message(&self) -> &HashSet<usize> {
        return &self.messages;
    }

    pub async fn broadcast(&mut self, message: usize) {
        let pending_request = &mut self.pending_request;
        let counter = &mut self.counter;
        let current_node_id = &NODE.get().unwrap().id;

        for other_node_id in self.topology.get().iter() {
            let msg_id = counter.generate_unique_msg_id();
            let req_id = counter.generate_unique_msg_id();
            let message = Message::new(
                current_node_id.clone(),
                other_node_id.clone(),
                MessageBody::node_broadcast {
                    message,
                    msg_id,
                    req_id,
                },
            );

            message.send(pending_request).await;
        }
    }

    pub fn accept_broadcast_response(&mut self, msg_id: usize) {
        self.pending_request.resolve_pending_request(msg_id);
    }
}
