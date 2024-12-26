use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_gossip(req_message: &Message, node: Arc<Mutex<Node>>) -> Message {
    if let MessageTypeData::Gossip { message, already_spread } = &req_message.body.type_specific {
        let mut node = node.lock().unwrap();

        let mut res = node.reply_to(&req_message);

        node.messages.insert(*message);

        let neighbors = node.neighbors.clone();

        let diff = neighbors.difference(&already_spread);

        let new_already_sent: HashSet<&String> = neighbors.union(&already_spread).collect();

        let mut  new_already_sent: HashSet<String> = new_already_sent.iter().map(|item| item.to_owned().clone()).collect();

        new_already_sent.insert(node.id.clone().unwrap());

        for dest in diff {
            let mut gossip = node.new_message(dest.clone());
            gossip.body.type_specific = MessageTypeData::Gossip {
                message: *message,
                already_spread: new_already_sent.clone(),
            };

            println!("{}", serde_json::to_string(&gossip).unwrap());
        }

        res.body.type_specific = MessageTypeData::GossipOk {};

        return res;
    }

    unreachable!();
}
