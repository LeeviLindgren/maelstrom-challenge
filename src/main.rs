use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    #[serde(rename = "type")]
    ty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<i64>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum Payload {
    Echo { echo: String },
}

fn main() -> Result<()> {
    let msg = Message {
        src: "c1".into(),
        dest: "n1".into(),
        body: Body {
            ty: "echo".into(),
            msg_id: Some(1),
            in_reply_to: None,
            payload: Payload::Echo {
                echo: "Muna pelle".into(),
            },
        },
    };
    let json = serde_json::to_string_pretty(&msg).context("Serialize a message")?;
    println!("{}", json);
    Ok(())
}
