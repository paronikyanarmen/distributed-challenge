use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_read(message: &Message, node: &mut Node) -> Message {
    if let MessageTypeData::Read {} = &message.body.type_specific {
        let mut res = node.reply_to(&message);


        res.body.type_specific = MessageTypeData::ReadOk {
            messages: node.messages.clone(),
        };

        return res;
    }

    panic!("Wrong message type");
}
