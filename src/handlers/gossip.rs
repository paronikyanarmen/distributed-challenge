use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_gossip(req_message: &Message, node: &mut Node) -> Message {
    if let MessageTypeData::Gossip { message, already_spread } = &req_message.body.type_specific {
        let mut res = node.reply_to(&req_message);

        node.messages.insert(*message);

        for dest in node.neighbors.clone() {
            let mut gossip = node.new_message(dest.clone());
            gossip.body.type_specific = MessageTypeData::Gossip {
                message: *message,
                already_spread: node.neighbors.clone(),
            };

            println!("{}", serde_json::to_string(&gossip).unwrap());
        }

        res.body.type_specific = MessageTypeData::GossipOk {};

        return res;
    }

    panic!("Wrong message type");
}
