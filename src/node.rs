use crate::message::{Message, MessageBody, MessageTypeData};
use std::collections::HashSet;

pub struct Node {
    pub id: String,
    message_id: usize,
    pub messages: HashSet<u64>,
    pub neighbors: HashSet<String>,
}

impl Node {

    pub fn from(message: &Message) -> Node {
        if let MessageTypeData::Init { node_id } = &message.body.type_specific {
           return Node {
               id: node_id.clone(),
               message_id: 0,
               messages: HashSet::new(),
               neighbors: HashSet::new(),
           }
        }
        unreachable!();
    }

    pub fn reply_to(&mut self, message: &Message) -> Message {
        let mut body = message.body.clone();

        body.in_reply_to = body.msg_id;

        body.msg_id = Some(self.message_id as u64);

        let res = Message {
            src: self.id.clone(),
            dest: message.src.clone(),
            body,
        };

        self.message_id += 1;

        res
    }

    pub fn new_message(&mut self, dest: String) -> Message {
        let body = MessageBody {
            msg_id: Some(self.message_id as u64),
            in_reply_to: None,
            type_specific: MessageTypeData::InitOk {},
        };

        let res = Message {
            src: self.id.clone(),
            dest,
            body,
        };

        self.message_id += 1;

        res
    }

    pub fn get_message_id(&self) -> usize {
        self.message_id
    }
}