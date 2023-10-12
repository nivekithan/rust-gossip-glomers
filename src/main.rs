use message::{Message, MessageBody, ResponseBody};
use tokio::io::{AsyncBufReadExt, BufReader};

mod message;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    while let Some(line) = lines.next_line().await? {
        let message: Message = serde_json::from_str(&line)?;

        match &message.body {
            MessageBody::echo { echo, .. } => {
                let response = message.response(&ResponseBody::echo_ok { echo: echo.clone() });
                response.send().unwrap();
            }

            MessageBody::init { .. } => {
                let response = message.response(&ResponseBody::init_ok {});
                response.send().unwrap();
            }

            MessageBody::init_ok { .. } | MessageBody::echo_ok { .. } => unreachable!(),
        }
    }

    return Ok(());
}
