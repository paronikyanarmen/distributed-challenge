use echo_challenge::handlers::{handle_broadcast, handle_gossip, handle_init, handle_read, handle_topology};
use echo_challenge::message::{Message, MessageTypeData};
use echo_challenge::node::Node;
use std::io;

fn main() -> io::Result<()> {
    let deserializer = serde_json::Deserializer::from_reader(io::stdin());
    let de_iter = deserializer.into_iter::<Message>();

    let mut node = Node::new();

    for message in de_iter {
        let message = message?;

        let res = match message.body.type_specific {
            MessageTypeData::Init { .. } => handle_init(&message, &mut node),
            MessageTypeData::Broadcast { .. } => handle_broadcast(&message, &mut node),
            MessageTypeData::Read {} => handle_read(&message, &mut node),
            MessageTypeData::Topology { .. } => handle_topology(&message, &mut node),
            MessageTypeData::Gossip { .. } => handle_gossip(&message, &mut node),
            _ => message
        };

        println!("{}", serde_json::to_string(&res)?);
    }
    Ok(())
}
