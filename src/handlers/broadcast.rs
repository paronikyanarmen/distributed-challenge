use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_broadcast(req_message: &Message, node: &mut Node) -> Message {
    if let MessageTypeData::Broadcast { message} = &req_message.body.type_specific {
        let mut res = node.reply_to(&req_message);

        node.messages.push(*message);

        res.body.type_specific = MessageTypeData::BroadcastOk {};

        return res;
    }

    panic!("Wrong message type");
}
