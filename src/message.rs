use serde_json::Result;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    src: String,
    dest: String,
    pub body: MessageBody,
}

impl Message {
    fn msg_id(&self) -> usize {
        match self.body {
            MessageBody::echo { msg_id, echo: _ } => return msg_id,
            MessageBody::init {
                msg_id,
                node_id: _,
                node_ids: _,
            } => return msg_id,
            MessageBody::init_ok { .. } | MessageBody::echo_ok { .. } => unreachable!(),
        }
    }
    pub fn response(&self, body: &ResponseBody) -> Message {
        match body {
            ResponseBody::echo_ok { echo } => {
                let response_message = Message {
                    src: self.dest.clone(),
                    dest: self.src.clone(),
                    body: MessageBody::echo_ok {
                        in_reply_to: self.msg_id(),
                        echo: echo.clone(),
                    },
                };

                return response_message;
            }
            ResponseBody::init_ok {} => {
                let response_message = Message {
                    src: self.dest.clone(),
                    dest: self.src.clone(),
                    body: MessageBody::init_ok {
                        in_reply_to: self.msg_id(),
                    },
                };

                return response_message;
            }
        }
    }

    pub fn send(&self) -> Result<()> {
        let serialized_form = serde_json::to_string(self)?;

        eprintln!("[OUTPUT] {serialized_form}");
        println!("{serialized_form}");

        return Ok(());
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
    init_ok {
        in_reply_to: usize,
    },
    echo {
        msg_id: usize,
        echo: String,
    },
    echo_ok {
        in_reply_to: usize,
        echo: String,
    },
}

#[allow(non_camel_case_types)]
pub enum ResponseBody {
    echo_ok { echo: String },
    init_ok {},
}
