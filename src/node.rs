use crate::message::{Message, MessageBody};

pub struct Node {
    pub id: Option<String>,
    pub message_id: usize,
}

impl Node {
    pub fn reply_to(&mut self, message: &Message, message_body: MessageBody) -> Message {

        let mut res = Message {
            src: self.id.clone().unwrap(),
            dest: message.src.clone(),
            body: message_body,
        };

        self.message_id += 1;

        res
    }
}