use std::sync::{Arc, Mutex};
use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_init(message: &Message, node: Arc<Mutex<Node>>) -> Message {
    if let MessageTypeData::Init { node_id } = &message.body.type_specific {
        let mut node = node.lock().unwrap();

        let node_id = node_id.clone().unwrap();

        node.id = Some(node_id);

        let mut res = node.reply_to(&message);

        res.body.type_specific = MessageTypeData::InitOk {};

        return res;
    }

    unreachable!();
}
