use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Response {
    src: String,
    dest: String,
    pub body: ResponseBody,
}

impl Response {
    pub fn from_message(message: &Message, response_body: ResponseBody) -> Response {
        let response = Response {
            src: message.dest.clone(),
            dest: message.src.clone(),
            body: response_body,
        };

        return response;
    }

    pub fn send(&self) {
        let self_in_str = serde_json::to_string(self).unwrap();
        println!("{self_in_str}");
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    src: String,
    dest: String,
    pub body: MessageBody,
}

impl Message {
    pub fn new(src: String, dest: String, body: MessageBody) -> Self {
        return Self { src, dest, body };
    }
    fn msg_id(&self) -> usize {
        match self.body {
            MessageBody::echo { msg_id, .. }
            | MessageBody::init { msg_id, .. }
            | MessageBody::generate { msg_id, .. }
            | MessageBody::topology { msg_id, .. }
            | MessageBody::broadcast { msg_id, .. }
            | MessageBody::read { msg_id } => return msg_id,

            MessageBody::node_broadcast { .. } => unreachable!(),
        }
    }

    pub fn send(&self) {
        let message = serde_json::to_string(self).unwrap();

        println!("{message}");
    }
    pub fn respond_with_echo_ok(&self, echo: String) {
        let response_message = Response::from_message(
            self,
            ResponseBody::echo_ok {
                in_reply_to: self.msg_id(),
                echo,
            },
        );

        response_message.send();
    }

    pub fn respond_with_init_ok(&self) {
        let response_message = Response::from_message(
            self,
            ResponseBody::init_ok {
                in_reply_to: self.msg_id(),
            },
        );

        response_message.send();
    }

    pub fn respond_with_generate_ok(&self, id: String) {
        let response_message = Response::from_message(
            self,
            ResponseBody::generate_ok {
                id,
                in_reply_to: self.msg_id(),
            },
        );

        response_message.send();
    }

    pub fn respond_with_topology_ok(&self) {
        let response_message = Response::from_message(
            self,
            ResponseBody::topology_ok {
                in_reply_to: self.msg_id(),
            },
        );

        response_message.send();
    }

    pub fn respond_with_broadcast_ok(&self) {
        let response_message = Response::from_message(
            self,
            ResponseBody::broadcast_ok {
                in_reply_to: self.msg_id(),
            },
        );

        response_message.send();
    }

    pub fn respond_with_read_ok(&self, messages: Vec<usize>) {
        let response_message = Response::from_message(
            self,
            ResponseBody::read_ok {
                messages,
                in_reply_to: self.msg_id(),
            },
        );

        response_message.send();
    }
}

#[allow(non_camel_case_types)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum MessageBody {
    init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },

    echo {
        msg_id: usize,
        echo: String,
    },

    generate {
        msg_id: usize,
    },

    topology {
        topology: HashMap<String, Vec<String>>,
        msg_id: usize,
    },

    broadcast {
        message: usize,
        msg_id: usize,
    },

    node_broadcast {
        message: usize,
    },

    read {
        msg_id: usize,
    },
}

#[allow(non_camel_case_types)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ResponseBody {
    init_ok {
        in_reply_to: usize,
    },
    echo_ok {
        in_reply_to: usize,
        echo: String,
    },
    generate_ok {
        id: String,
        in_reply_to: usize,
    },
    read_ok {
        messages: Vec<usize>,
        in_reply_to: usize,
    },
    broadcast_ok {
        in_reply_to: usize,
    },
    topology_ok {
        in_reply_to: usize,
    },
}
