use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_init(message: &Message, node: &mut Node) -> Message {
    if let MessageTypeData::Init { .. } = &message.body.type_specific {
        let mut res = node.reply_to(&message);

        res.body.type_specific = MessageTypeData::InitOk {};

        return res;
    }

    unreachable!();
}
