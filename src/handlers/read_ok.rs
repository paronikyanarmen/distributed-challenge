use std::sync::{Arc, Mutex};
use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_read_ok(message: &Message, node: Arc<Mutex<Node>>) {

    eprintln!("{:?}", message);
    if let MessageTypeData::ReadOk {messages} = &message.body.type_specific {
        let mut node = node.lock().unwrap();

        node.messages.extend(messages.iter().cloned());
        return;
    }

    unreachable!();
}
