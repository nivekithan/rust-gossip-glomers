use broadcast::BroadcastService;
use counter::Counter;
use message::{Message, MessageBody};
use node::{Node, NODE};
use tokio::io::{AsyncBufReadExt, BufReader};

mod broadcast;
mod counter;
mod message;
mod node;
mod topology;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let mut counter = Counter::new();
    let mut broadcast = BroadcastService::new();

    while let Some(line) = lines.next_line().await? {
        let message: Message = serde_json::from_str(&line)?;

        match &message.body {
            MessageBody::echo { echo, .. } => {
                message.respond_with_echo_ok(echo.to_string());
            }

            MessageBody::init { node_id, .. } => {
                NODE.get_or_init(|| {
                    broadcast.topology.set_id(node_id.to_string());
                    return Node::new(node_id);
                });

                message.respond_with_init_ok();
            }

            MessageBody::generate { .. } => {
                let unique_id = counter.generate_unique_id(&NODE.get().unwrap());

                message.respond_with_generate_ok(unique_id);
            }

            MessageBody::topology { topology, .. } => {
                broadcast.topology.set_topology(topology.clone());
                message.respond_with_topology_ok();
            }

            MessageBody::broadcast {
                message: broadcast_message,
                ..
            } => {
                broadcast.add_message(*broadcast_message);

                message.respond_with_broadcast_ok();
            }

            MessageBody::read { .. } => {
                let messages = broadcast.get_message();

                message.respond_with_read_ok(Vec::from_iter(messages.iter().map(|v| *v)));
            }

            MessageBody::node_broadcast { message } => {
                broadcast.add_message(*message);
            }
        }
    }

    return Ok(());
}
