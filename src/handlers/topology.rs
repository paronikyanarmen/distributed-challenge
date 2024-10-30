use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_topology(req_message: &Message, node: &mut Node) -> Message {
    if let MessageTypeData::Topology { .. } = &req_message.body.type_specific {
        let mut res = node.reply_to(&req_message);

        res.body.type_specific = MessageTypeData::TopologyOk {};

        return res;
    }

    panic!("Wrong message type");
}
