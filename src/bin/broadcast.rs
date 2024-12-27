use echo_challenge::handlers::{handle_broadcast, handle_gossip, handle_init, handle_read, handle_topology};
use echo_challenge::message::{Message, MessageTypeData};
use echo_challenge::node::Node;
use std::{io, thread};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use echo_challenge::background::check_neighbors;

fn main() -> io::Result<()> {
    let deserializer = serde_json::Deserializer::from_reader(io::stdin());
    let mut de_iter = deserializer.into_iter::<Message>();

    let init = de_iter.next().unwrap()?;

    let mut node = Node::from(&init);

    let init_response = handle_init(&init, &mut node);

    println!("{}", serde_json::to_string(&init_response)?);

    let node = Arc::new(Mutex::new(node));

    let node_copy  = Arc::clone(&node);

    thread::spawn(move || {
        check_neighbors(node_copy);
    });

    for message in de_iter {
        let message = message?;

        let res = match message.body.type_specific {
            MessageTypeData::Broadcast { .. } => handle_broadcast(&message, Arc::clone(&node)),
            MessageTypeData::Read {} => handle_read(&message, Arc::clone(&node)),
            MessageTypeData::Topology { .. } => handle_topology(&message, Arc::clone(&node)),
            MessageTypeData::Gossip { .. } => handle_gossip(&message, Arc::clone(&node)),
            _ => continue
        };

        println!("{}", serde_json::to_string(&res)?);
        thread::sleep(Duration::from_micros(1));
    }

    Ok(())
}
