use std::collections::HashMap;
use tokio::sync::oneshot;

pub struct PendingRequestService {
    pending_request: HashMap<usize, oneshot::Sender<()>>,
}

impl PendingRequestService {
    pub fn new() -> Self {
        return Self {
            pending_request: HashMap::new(),
        };
    }

    pub fn add_pending_request(&mut self, req_id: usize, sender: oneshot::Sender<()>) {
        self.pending_request.insert(req_id, sender);
    }

    pub fn resolve_pending_request(&mut self, req_id: usize) {
        let sender = self.pending_request.remove(&req_id);

        if let Some(sender) = sender {
            sender.send(()).unwrap();
        }
    }

    pub fn remove_request(&mut self, req_id: usize) {
        self.pending_request.remove(&req_id);
    }
}
