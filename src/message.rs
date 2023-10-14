use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    src: String,
    dest: String,
    pub body: MessageBody,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Response {
    src: String,
    dest: String,
    pub body: ResponseMessageBody,
}

impl Response {
    pub fn send(&self) {
        let self_in_str = serde_json::to_string(self).unwrap();
        println!("{self_in_str}");
    }
}

impl Message {
    fn msg_id(&self) -> usize {
        match self.body {
            MessageBody::echo { msg_id, .. }
            | MessageBody::init { msg_id, .. }
            | MessageBody::generate { msg_id, .. }
            | MessageBody::topology { msg_id, .. }
            | MessageBody::broadcast { msg_id, .. }
            | MessageBody::read { msg_id } => return msg_id,
        }
    }
    pub fn response(&self, body: &ResponseBody) -> Response {
        match body {
            ResponseBody::echo_ok { echo } => {
                let response_message = Response {
                    src: self.dest.clone(),
                    dest: self.src.clone(),
                    body: ResponseMessageBody::echo_ok {
                        in_reply_to: self.msg_id(),
                        echo: echo.clone(),
                    },
                };

                return response_message;
            }
            ResponseBody::init_ok {} => {
                let response_message = Response {
                    src: self.dest.clone(),
                    dest: self.src.clone(),
                    body: ResponseMessageBody::init_ok {
                        in_reply_to: self.msg_id(),
                    },
                };

                return response_message;
            }
            ResponseBody::generate_ok { id } => {
                let response_message = Response {
                    src: self.dest.clone(),
                    dest: self.src.clone(),
                    body: ResponseMessageBody::generate_ok {
                        id: id.clone(),
                        in_reply_to: self.msg_id(),
                    },
                };

                return response_message;
            }

            ResponseBody::topology_ok {} => {
                let response_message = Response {
                    src: self.dest.clone(),
                    dest: self.src.clone(),
                    body: ResponseMessageBody::topology_ok {
                        in_reply_to: self.msg_id(),
                    },
                };

                return response_message;
            }

            ResponseBody::broadcast_ok {} => {
                let response_message = Response {
                    src: self.dest.clone(),
                    dest: self.src.clone(),
                    body: ResponseMessageBody::broadcast_ok {
                        in_reply_to: self.msg_id(),
                    },
                };

                return response_message;
            }

            ResponseBody::read_ok { messages } => {
                let response_message = Response {
                    src: self.dest.clone(),
                    dest: self.src.clone(),
                    body: ResponseMessageBody::read_ok {
                        messages: (*messages).clone(),
                        in_reply_to: self.msg_id(),
                    },
                };

                return response_message;
            }
        }
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

    read {
        msg_id: usize,
    },
}

#[allow(non_camel_case_types)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ResponseMessageBody {
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

#[allow(non_camel_case_types)]
pub enum ResponseBody<'a> {
    echo_ok { echo: String },
    init_ok {},
    generate_ok { id: String },
    topology_ok {},
    broadcast_ok {},
    read_ok { messages: &'a Vec<usize> },
}
