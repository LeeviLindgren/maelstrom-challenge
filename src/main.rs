use anyhow::{Context, Result};
use maelstrom_challenge::{Message, Node};

fn main() -> Result<()> {
    let node = Node;
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let inputs = serde_json::Deserializer::from_reader(stdin);

    for item in inputs.into_iter::<Message>() {
        let msg = item.context("Deserializing message from STDIN.")?;
        node.handle(msg, &mut stdout)
            .context("Handling the received message.")?;
    }

    Ok(())
}
