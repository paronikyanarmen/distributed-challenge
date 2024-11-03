use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_topology(req_message: &Message, node: &mut Node) -> Message {
    if let MessageTypeData::Topology { data } = &req_message.body.type_specific {
        let mut res = node.reply_to(&req_message);

        let node_id = node.id.as_ref().unwrap();

        for other_id in data["topology"][node_id].as_array().unwrap() {
            let other_id = other_id.to_string();
            let other_id = &other_id[1..other_id.len() - 1];
            node.neighbors.insert(other_id.to_string());
        }

        res.body.type_specific = MessageTypeData::TopologyOk {};

        return res;
    }

    panic!("Wrong message type");
}
