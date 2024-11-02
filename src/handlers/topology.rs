use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_topology(req_message: &Message, node: &mut Node) -> Message {
    if let MessageTypeData::Topology { data } = &req_message.body.type_specific {
        let mut res = node.reply_to(&req_message);

        let node_id = node.id.as_ref().unwrap();

        for other_id in data["topology"][node_id].as_array().unwrap() {
            node.neighbors.push(other_id.to_string());
        }

        res.body.type_specific = MessageTypeData::TopologyOk {};

        return res;
    }

    panic!("Wrong message type");
}
