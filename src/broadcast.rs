pub struct BroadcastMessages {
    messages: Vec<usize>,
}

impl BroadcastMessages {
    pub fn new() -> Self {
        return BroadcastMessages {
            messages: Vec::new(),
        };
    }

    pub fn add(&mut self, new_message: usize) {
        self.messages.push(new_message);
    }

    pub fn get(&self) -> &Vec<usize> {
        return &self.messages;
    }
}
