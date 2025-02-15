use crate::message::MessageTypeData;
use crate::node::Node;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn check_neighbors(node: Arc<Mutex<Node>>) {

    loop {
        let mut node = node.lock().unwrap();

        let neighbors = node.neighbors.clone();

        for neighbor in neighbors {
            let mut message = node.new_message(neighbor);
            message.body.type_specific = MessageTypeData::Read {};
            println!("{}", serde_json::to_string(&message).unwrap());
        }
        drop(node);
        thread::sleep(Duration::from_millis(500));
    }
}
