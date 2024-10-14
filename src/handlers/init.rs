use crate::message::{Message, MessageBody, MessageMeta};
use crate::node::Node;

pub fn handle_init(message: &Message, node: &mut Node) -> Message {
    if let MessageBody::Init { node_id, meta } = &message.body {
        let mut res = message.clone();

        let node_id = node_id.clone().unwrap();
        node.id = Some(node_id.clone());

        res.src = node_id;
        res.dest = message.src.clone();

        res.body = MessageBody::InitOk {
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
