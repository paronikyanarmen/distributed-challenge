use crate::message::{Message, MessageBody, MessageMeta};
use crate::node::Node;

pub fn handle_echo(message: &Message, node: &mut Node) -> Message {
    if let MessageBody::Echo { echo, meta } = &message.body {
        let mut res = message.clone();

        res.src = node.id.clone().unwrap();
        res.dest = message.src.clone();

        res.body = MessageBody::EchoOk {
            echo: echo.to_string(),
            meta: MessageMeta {
                msg_id: Some(node.message_id as u64),
                in_reply_to: meta.msg_id,
            },
        };

        node.message_id += 1;

        return res;
    }

    panic!("Wrong message type");
}
