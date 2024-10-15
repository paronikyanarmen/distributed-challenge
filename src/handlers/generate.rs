use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_generate(message: &Message, node: &mut Node) -> Message {
    if let MessageTypeData::Generate {} = &message.body.type_specific {
        let mut res = node.reply_to(&message);

        let mut id = node.id.clone().unwrap();

        id.push_str(node.message_id.to_string().as_str());

        res.body.type_specific = MessageTypeData::GenerateOk {
            id,
        };

        return res;
    }

    panic!("Wrong message type");
}
