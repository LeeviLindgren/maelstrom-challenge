/// Models for inter node communication
///
/// See the `Maelstrom` documentation for detailed description of the communication
/// [protocol](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md).
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
/// Nodes communicate with `Message`s
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Payload,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
/// Body of each message
///
/// Each body contains a payload and the `Payload` enum variants
/// correspond to each possible message type a node can receive.
pub enum Payload {
    /// Echo command
    Echo { msg_id: i64, echo: String },
    /// Respond to echo command with EchoOk
    EchoOk {
        echo: String,
        in_reply_to: i64,
        msg_id: i64,
    },
    /// Generate unique id command
    Generate { msg_id: i64 },
    /// Respond with `GenerateOk` and the `id`
    GenerateOk {
        msg_id: i64,
        in_reply_to: i64,
        id: Uuid,
    },
    /// `Init` command, which is the first thing a node receives.
    Init {
        msg_id: i64,
        node_id: String,
        node_ids: Vec<String>,
    },
    /// Response to the `Init` command.
    InitOk { in_reply_to: i64 },
}
