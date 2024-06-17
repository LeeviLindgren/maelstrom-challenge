use std::io::Stdout;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    src: String,
    dest: String,
    body: Payload,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Payload {
    Echo {
        msg_id: i64,
        echo: String,
    },
    EchoOk {
        echo: String,
        in_reply_to: i64,
        msg_id: i64,
    },
    Init {
        msg_id: i64,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: i64,
    },
}

struct Node;

impl Node {
    fn handle(&self, msg: Message, writer: &Stdout) -> Result<()> {
        let response = Message {
            src: msg.dest,
            dest: msg.src,
            body: match msg.body {
                Payload::Echo { msg_id, echo } => Payload::EchoOk {
                    in_reply_to: msg_id,
                    echo,
                    msg_id,
                },
                Payload::Init { msg_id, .. } => Payload::InitOk {
                    in_reply_to: msg_id,
                },
                _ => bail!("Invalid input message."),
            },
        };

        serde_json::to_writer_pretty(writer, &response).context("Serializing to STDOUT")?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let node = Node;
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let inputs = serde_json::Deserializer::from_reader(stdin);

    for item in inputs.into_iter::<Message>() {
        let msg = item.context("Deserializing message from STDIN.")?;
        node.handle(msg, &stdout)
            .context("Handling the received message.")?;
    }

    Ok(())
}
