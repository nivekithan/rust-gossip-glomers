use broadcast::BroadcastMessages;
use counter::Counter;
use message::{Message, MessageBody, ResponseBody};
use node::{Node, NODE};
use tokio::io::{AsyncBufReadExt, BufReader};

mod broadcast;
mod counter;
mod message;
mod node;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let mut counter = Counter::new();
    let mut broadcast = BroadcastMessages::new();

    while let Some(line) = lines.next_line().await? {
        let message: Message = serde_json::from_str(&line)?;

        match &message.body {
            MessageBody::echo { echo, .. } => {
                let response = message.response(&ResponseBody::echo_ok { echo: echo.clone() });
                response.send().unwrap();
            }

            MessageBody::init { node_id, .. } => {
                let response = message.response(&ResponseBody::init_ok {});
                NODE.get_or_init(|| Node::new(node_id));
                response.send().unwrap();
            }

            MessageBody::generate { .. } => {
                let unique_id = counter.generate_unique_id(&NODE.get().unwrap());

                let response = message.response(&ResponseBody::generate_ok { id: unique_id });
                response.send().unwrap();
            }

            MessageBody::topology { .. } => {
                let response = message.response(&ResponseBody::topology_ok {});
                response.send().unwrap();
            }

            MessageBody::broadcast {
                message: broadcast_message,
                ..
            } => {
                broadcast.add(*broadcast_message);

                let response = message.response(&ResponseBody::broadcast_ok {});
                response.send().unwrap();
            }

            MessageBody::read { .. } => {
                let messages = broadcast.get();

                let response = message.response(&ResponseBody::read_ok { messages });
                response.send().unwrap();
            }

            MessageBody::init_ok { .. }
            | MessageBody::echo_ok { .. }
            | MessageBody::generate_ok { .. }
            | MessageBody::topology_ok { .. }
            | MessageBody::broadcast_ok { .. }
            | MessageBody::read_ok { .. } => unreachable!(),
        }
    }

    return Ok(());
}
