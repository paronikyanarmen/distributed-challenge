use std::collections::HashSet;
use crate::message::Message;

pub struct Node {
    pub id: Option<String>,
    pub message_id: usize,
    pub messages: HashSet<u64>,
    pub neighbors: Vec<String>
}

impl Node {
    pub fn new() -> Node {
        Self {
            id: None,
            message_id: 1,
            messages: HashSet::new(),
            neighbors: Vec::new()
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
}