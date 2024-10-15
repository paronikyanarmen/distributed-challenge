use crate::message::{Message, MessageTypeData};
use crate::node::Node;

pub fn handle_echo(message: &Message, node: &mut Node) -> Message {
    if let MessageTypeData::Echo { echo } = &message.body.type_specific {
        let mut res = node.reply_to(&message);

        res.body.type_specific = MessageTypeData::EchoOk {
            echo: echo.to_string(),
        };

        return res;
    }

    panic!("Wrong message type");
}
