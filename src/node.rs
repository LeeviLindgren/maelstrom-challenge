use crate::message::{Message, Payload};
use anyhow::{bail, Context, Result};
use std::io::{StdoutLock, Write};
use uuid::Uuid;

pub struct Node;

impl Node {
    fn generate_uuid7() -> Uuid {
        Uuid::now_v7()
    }

    pub fn handle(&self, msg: Message, writer: &mut StdoutLock) -> Result<()> {
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
                Payload::Generate { msg_id } => Payload::GenerateOk {
                    msg_id,
                    in_reply_to: msg_id,
                    id: Node::generate_uuid7(),
                },
                _ => bail!("Invalid input message."),
            },
        };

        // Deref writer and make a mutable reference again.
        // Write is implemented only for &mut StdoutLock
        serde_json::to_writer(&mut *writer, &response).context("Serializing to STDOUT")?;
        writer.write_all(b"\n").context("Write newline to STDOUT")?;
        Ok(())
    }
}
