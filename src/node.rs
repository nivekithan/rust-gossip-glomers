use std::sync::OnceLock;

pub struct Node {
    pub id: String,
}

impl Node {
    pub fn new(id: &str) -> Self {
        return Node { id: id.to_string() };
    }
}

pub static NODE: OnceLock<Node> = OnceLock::new();
