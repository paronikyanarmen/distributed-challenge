use crate::message::{Message, MessageBody, MessageTypeData};
use std::collections::HashSet;

pub struct Node {
    pub id: Option<String>,
    message_id: usize,
    pub messages: HashSet<u64>,
    pub neighbors: HashSet<String>,
}

impl Node {
    pub fn new() -> Node {
        Self {
            id: None,
            message_id: 1,
            messages: HashSet::new(),
            neighbors: HashSet::new(),
        }
    }

    pub fn reply_to(&mut self, message: &Message) -> Message {
        let mut body = message.body.clone();

        body.in_reply_to = body.msg_id;

        body.msg_id = Some(self.message_id as u64);

        let res = Message {
            src: self.id.clone().unwrap(),
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
            src: self.id.clone().unwrap(),
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