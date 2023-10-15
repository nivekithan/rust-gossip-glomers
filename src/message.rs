use std::{collections::HashMap, sync::Weak, time::Duration};

use tokio::{sync::oneshot, time::timeout};

use crate::{counter::Counter, pending_request::PendingRequestService};

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

    pub fn send_and_forget(&self) {
        let self_in_str = serde_json::to_string(self).unwrap();
        eprintln!("[OUTPUT] {self_in_str}");
        println!("{self_in_str}");
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
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
        let msg_id = self.try_msg_id();

        return msg_id.unwrap();
    }

    fn try_msg_id(&self) -> Option<usize> {
        match self.body {
            MessageBody::echo { msg_id, .. }
            | MessageBody::init { msg_id, .. }
            | MessageBody::generate { msg_id, .. }
            | MessageBody::topology { msg_id, .. }
            | MessageBody::broadcast { msg_id, .. }
            | MessageBody::read { msg_id }
            | MessageBody::node_broadcast { msg_id, .. } => return Some(msg_id),

            MessageBody::node_broadcast_ok { .. } => None,
        }
    }

    pub fn send_and_forget(&self) {
        let message = serde_json::to_string(self).unwrap();

        eprintln!("[OUTPUT] {message}");
        println!("{message}");
    }

    pub async fn send(
        &self,
        pending_request_service: &mut PendingRequestService,
        counter: Weak<Counter>,
    ) {
        let (sender, mut receiver) = oneshot::channel::<()>();

        let message_id = self.msg_id();
        let message = serde_json::to_string(self).unwrap();

        pending_request_service.add_pending_request(message_id, sender);

        eprintln!("[OUTPUT] {message}");
        println!("{message}");

        let cloned_message = (*self).clone();
        tokio::task::spawn(async move {
            let message = cloned_message;
            let counter = counter.upgrade().unwrap();

            while let Err(_) = timeout(Duration::from_millis(100), &mut receiver).await {
                let message = message.new_msg_id(&counter);
                let message = serde_json::to_string(&message).unwrap();

                eprintln!("[OUTPUT] {message}");
                println!("{message}");
            }
        });
    }

    pub fn new_msg_id(&self, counter: &Counter) -> Self {
        match self.body {
            MessageBody::node_broadcast { message,req_id, .. } => {
                return Message {
                    src: self.src.clone(),
                    dest: self.dest.clone(),
                    body: MessageBody::node_broadcast { message , req_id, msg_id: counter.generate_unique_msg_id()  }
                }
            },

            _ => panic!("Can calll remove_msg_id only when Message.body is of variant MessageBody::node_broadcast"),
        }
    }

    pub fn respond_with_echo_ok(&self, echo: String) {
        let response_message = Response::from_message(
            self,
            ResponseBody::echo_ok {
                in_reply_to: self.msg_id(),
                echo,
            },
        );

        response_message.send_and_forget();
    }

    pub fn respond_with_init_ok(&self) {
        let response_message = Response::from_message(
            self,
            ResponseBody::init_ok {
                in_reply_to: self.msg_id(),
            },
        );

        response_message.send_and_forget();
    }

    pub fn respond_with_generate_ok(&self, id: String) {
        let response_message = Response::from_message(
            self,
            ResponseBody::generate_ok {
                id,
                in_reply_to: self.msg_id(),
            },
        );

        response_message.send_and_forget();
    }

    pub fn respond_with_topology_ok(&self) {
        let response_message = Response::from_message(
            self,
            ResponseBody::topology_ok {
                in_reply_to: self.msg_id(),
            },
        );

        response_message.send_and_forget();
    }

    pub fn respond_with_broadcast_ok(&self) {
        let response_message = Response::from_message(
            self,
            ResponseBody::broadcast_ok {
                in_reply_to: self.msg_id(),
            },
        );

        response_message.send_and_forget();
    }

    pub fn respond_with_read_ok(&self, messages: Vec<usize>) {
        let response_message = Response::from_message(
            self,
            ResponseBody::read_ok {
                messages,
                in_reply_to: self.msg_id(),
            },
        );

        response_message.send_and_forget();
    }

    pub fn respond_with_node_broadcast_ok(&self) {
        let response_message = Message::new(
            self.dest.clone(),
            self.src.clone(),
            MessageBody::node_broadcast_ok {
                in_reply_to: self.msg_id(),
                for_req: match &self.body {
                    MessageBody::node_broadcast { req_id, .. } => *req_id,
                    _ => panic!("responsd_with_node_broadcast_ok is called for MessageBody whose variant is not MessageBody::node_broadcast"),
                },
            },
        );

        response_message.send_and_forget();
    }
}

#[allow(non_camel_case_types)]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
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

    read {
        msg_id: usize,
    },

    node_broadcast {
        message: usize,
        msg_id: usize,
        req_id: usize,
    },

    node_broadcast_ok {
        in_reply_to: usize,
        for_req: usize,
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
