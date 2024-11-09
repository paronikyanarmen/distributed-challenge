use echo_challenge::handlers::{handle_broadcast, handle_gossip, handle_init, handle_read, handle_topology};
use echo_challenge::message::{Message, MessageTypeData};
use echo_challenge::node::Node;
use std::{io, thread};
use std::sync::{Arc, Mutex};
use echo_challenge::background::check_neighbors;

fn main() -> io::Result<()> {
    let deserializer = serde_json::Deserializer::from_reader(io::stdin());
    let de_iter = deserializer.into_iter::<Message>();

    let node = Arc::new(Mutex::new(Node::new()));

    let node_copy  = Arc::clone(&node);

    thread::spawn(move || {
        check_neighbors(node_copy);
    });

    for message in de_iter {
        let message = message?;

        let res = match message.body.type_specific {
            MessageTypeData::Init { .. } => handle_init(&message, Arc::clone(&node)),
            MessageTypeData::Broadcast { .. } => handle_broadcast(&message, Arc::clone(&node)),
            MessageTypeData::Read {} => handle_read(&message, Arc::clone(&node)),
            MessageTypeData::Topology { .. } => handle_topology(&message, Arc::clone(&node)),
            MessageTypeData::Gossip { .. } => handle_gossip(&message, Arc::clone(&node)),
            _ => message
        };

        println!("{}", serde_json::to_string(&res)?);
    }
    Ok(())
}
